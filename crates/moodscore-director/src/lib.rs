//! Moodscore gesture director.
//!
//! Turns musical intent into motion. The agent says where things should go
//! and how long the trip should take; the director works out every value in
//! between and feeds it to the engine.
//!
//! Two kinds of thing move:
//!
//! - **Mood macros** (`darkness`, …), in `0..1`. Each drives several DSP
//!   parameters through mappings loaded from data ([`mapping`]).
//! - **Parameters**, directly, through [`Director::gesture`].
//!
//! Every parameter has one owner at a time: a macro, or manual control.
//! A `gesture` takes a parameter into manual control; a `morph` of its macro
//! takes it back, gliding from wherever it is to where the macro wants it.
//! No change ever starts from anywhere but the current value.
//!
//! [`Director`] is pure (time is passed in), which makes it easy to test.
//! [`Conductor`] runs it on a control thread against a live engine.

pub mod curve;
pub mod mapping;

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use moodscore_engine::{EngineHandle, ParamInfo, Scale};
use serde::Serialize;

pub use curve::Curve;
pub use mapping::{MacroDef, MappingError, DEFAULT_MAPPINGS};

/// The shortest a gesture can take, in seconds. "Instant" means this.
pub const MIN_DURATION: f32 = 0.05;

/// The longest a gesture can take, in seconds.
pub const MAX_DURATION: f32 = 600.0;

/// How often the control loop updates the engine.
pub const CONTROL_RATE_HZ: u32 = 500;

/// The standing default for how long transitions take and their shape, used
/// by every call that doesn't give its own. Set with [`Director::set_pace`].
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct Pace {
    /// Seconds. `0` means "as fast as is musically safe".
    pub over: f32,
    pub curve: Curve,
}

impl Default for Pace {
    fn default() -> Self {
        Self {
            over: 10.0,
            curve: Curve::EaseInOut,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DirectorError {
    UnknownMacro(String),
    UnknownParam(String),
    OutOfRange {
        name: String,
        value: f32,
        min: f32,
        max: f32,
    },
    Invalid(String),
}

impl fmt::Display for DirectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DirectorError::UnknownMacro(n) => write!(f, "unknown macro '{n}'"),
            DirectorError::UnknownParam(n) => write!(f, "unknown parameter '{n}'"),
            DirectorError::OutOfRange {
                name,
                value,
                min,
                max,
            } => write!(f, "{name} = {value} is outside {min} .. {max}"),
            DirectorError::Invalid(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for DirectorError {}

/// A value moving from `from` to `to`.
#[derive(Clone, Debug)]
struct Gesture {
    from: f32,
    to: f32,
    start: f64,
    duration: f32,
    curve: Curve,
}

impl Gesture {
    fn progress(&self, now: f64) -> f32 {
        let elapsed = (now - self.start) as f32;
        if elapsed <= 0.0 {
            0.0
        } else {
            self.curve.apply(elapsed / self.duration)
        }
    }

    fn remaining(&self, now: f64) -> f32 {
        ((self.start + self.duration as f64) - now).max(0.0) as f32
    }

    fn done(&self, now: f64) -> bool {
        now >= self.start + self.duration as f64
    }
}

/// Interpolate between `a` and `b`, geometrically when `log` (and both positive).
fn interp(a: f32, b: f32, k: f32, log: bool) -> f32 {
    if log && a > 0.0 && b > 0.0 {
        a * (b / a).powf(k)
    } else {
        a + (b - a) * k
    }
}

/// Something that can move: a held value, or a gesture in progress.
#[derive(Clone, Debug)]
struct Lane {
    value: f32,
    gesture: Option<Gesture>,
    log: bool,
}

impl Lane {
    fn new(value: f32, log: bool) -> Self {
        Self {
            value,
            gesture: None,
            log,
        }
    }

    fn at(&self, now: f64) -> f32 {
        match &self.gesture {
            Some(g) => interp(g.from, g.to, g.progress(now), self.log),
            None => self.value,
        }
    }

    /// Start moving toward `to`, from wherever the lane is right now.
    fn start(&mut self, now: f64, to: f32, start_in: f32, duration: f32, curve: Curve) {
        let from = self.at(now);
        self.value = from;
        self.gesture = Some(Gesture {
            from,
            to,
            start: now + start_in as f64,
            duration,
            curve,
        });
    }

    fn settle(&mut self, now: f64) {
        if let Some(g) = &self.gesture {
            if g.done(now) {
                self.value = g.to;
                self.gesture = None;
            }
        }
    }

    fn remaining(&self, now: f64) -> f32 {
        self.gesture.as_ref().map_or(0.0, |g| g.remaining(now))
    }

    fn target(&self) -> f32 {
        self.gesture.as_ref().map_or(self.value, |g| g.to)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Owner {
    Macro(usize),
    Manual,
}

/// Glide from a manual value back to a macro's control.
#[derive(Clone, Debug)]
struct Reclaim {
    from: f32,
    blend: Gesture,
}

#[derive(Clone, Debug)]
struct ParamLane {
    owner: Owner,
    manual: Lane,
    reclaim: Option<Reclaim>,
    /// Which macro drives this parameter in the mappings, if any.
    mapped_by: Option<(usize, usize)>,
    out: f32,
    log: bool,
}

/// The pure gesture engine. Time is passed in, in seconds.
pub struct Director {
    params: Vec<ParamInfo>,
    macros: Vec<MacroDef>,
    macro_lanes: Vec<Lane>,
    lanes: Vec<ParamLane>,
    pace: Pace,
}

impl Director {
    pub fn new(params: Vec<ParamInfo>, macros: Vec<MacroDef>) -> Self {
        let macro_lanes = macros.iter().map(|m| Lane::new(m.default, false)).collect();

        let mut lanes: Vec<ParamLane> = params
            .iter()
            .map(|p| {
                let log = p.scale == Scale::Log && p.min > 0.0;
                ParamLane {
                    owner: Owner::Manual,
                    manual: Lane::new(p.init, log),
                    reclaim: None,
                    mapped_by: None,
                    out: p.init,
                    log,
                }
            })
            .collect();

        for (mi, m) in macros.iter().enumerate() {
            for (ti, t) in m.targets.iter().enumerate() {
                let lane = &mut lanes[t.param];
                lane.mapped_by = Some((mi, ti));
                lane.owner = Owner::Macro(mi);
                lane.out = t.value(m.default);
            }
        }

        let mut d = Self {
            params,
            macros,
            macro_lanes,
            lanes,
            pace: Pace::default(),
        };
        d.tick(0.0);
        d
    }

    /// A director with the default mappings shipped in `macros/default.json`.
    pub fn with_default_mappings(params: Vec<ParamInfo>) -> Result<Self, MappingError> {
        let macros = mapping::load(DEFAULT_MAPPINGS, &params)?;
        Ok(Self::new(params, macros))
    }

    pub fn params(&self) -> &[ParamInfo] {
        &self.params
    }

    pub fn macros(&self) -> &[MacroDef] {
        &self.macros
    }

    /// The standing default for transitions.
    pub fn pace(&self) -> Pace {
        self.pace
    }

    /// Change the standing default for transitions. Omitted fields keep their
    /// current value. Movements already under way are not affected; the new
    /// pace applies to the next call that doesn't give its own `over` or curve.
    pub fn set_pace(
        &mut self,
        over: Option<f32>,
        curve: Option<Curve>,
    ) -> Result<Pace, DirectorError> {
        if over.is_none() && curve.is_none() {
            return Err(DirectorError::Invalid(
                "set_pace needs 'over', 'curve' or both".into(),
            ));
        }
        if let Some(o) = over {
            check_duration(o)?;
            self.pace.over = o;
        }
        if let Some(c) = curve {
            self.pace.curve = c;
        }
        Ok(self.pace)
    }

    /// Move one or more mood macros. Parameters a macro drives come back under
    /// its control, gliding over the same duration.
    pub fn morph(
        &mut self,
        now: f64,
        changes: &[(&str, f32)],
        over: f32,
        curve: Curve,
    ) -> Result<(), DirectorError> {
        let duration = check_duration(over)?;
        let mut resolved = Vec::with_capacity(changes.len());
        for (name, value) in changes {
            let mi = self
                .macros
                .iter()
                .position(|m| m.name == *name)
                .ok_or_else(|| DirectorError::UnknownMacro(name.to_string()))?;
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err(DirectorError::OutOfRange {
                    name: name.to_string(),
                    value: *value,
                    min: 0.0,
                    max: 1.0,
                });
            }
            resolved.push((mi, *value));
        }
        if resolved.is_empty() {
            return Err(DirectorError::Invalid(
                "morph needs at least one macro value".into(),
            ));
        }

        for (mi, value) in resolved {
            self.macro_lanes[mi].start(now, value, 0.0, duration, curve);
            for t in &self.macros[mi].targets {
                let lane = &mut self.lanes[t.param];
                if lane.owner != Owner::Macro(mi) || lane.reclaim.is_some() {
                    let from = lane.out;
                    lane.owner = Owner::Macro(mi);
                    lane.reclaim = Some(Reclaim {
                        from,
                        blend: Gesture {
                            from: 0.0,
                            to: 1.0,
                            start: now,
                            duration,
                            curve,
                        },
                    });
                }
            }
        }
        Ok(())
    }

    /// Move one parameter directly. Takes it out of macro control.
    pub fn gesture(
        &mut self,
        now: f64,
        path: &str,
        to: f32,
        over: f32,
        curve: Curve,
        start_in: f32,
    ) -> Result<(), DirectorError> {
        let duration = check_duration(over)?;
        if !start_in.is_finite() || !(0.0..=MAX_DURATION).contains(&start_in) {
            return Err(DirectorError::Invalid(format!(
                "start_in must be between 0 and {MAX_DURATION} seconds"
            )));
        }
        let id = self
            .params
            .iter()
            .position(|p| p.path == path)
            .ok_or_else(|| DirectorError::UnknownParam(path.to_string()))?;
        let p = &self.params[id];
        if !to.is_finite() || !(p.min..=p.max).contains(&to) {
            return Err(DirectorError::OutOfRange {
                name: path.to_string(),
                value: to,
                min: p.min,
                max: p.max,
            });
        }

        let lane = &mut self.lanes[id];
        if lane.owner != Owner::Manual || lane.reclaim.is_some() {
            lane.manual = Lane::new(lane.out, lane.log);
            lane.owner = Owner::Manual;
            lane.reclaim = None;
        }
        lane.manual.start(now, to, start_in, duration, curve);
        Ok(())
    }

    /// Advance to `now`. Read the result with [`values`](Self::values).
    pub fn tick(&mut self, now: f64) {
        let macro_values: Vec<f32> = self.macro_lanes.iter().map(|l| l.at(now)).collect();
        for l in &mut self.macro_lanes {
            l.settle(now);
        }

        for lane in &mut self.lanes {
            lane.out = match lane.owner {
                Owner::Manual => {
                    let v = lane.manual.at(now);
                    lane.manual.settle(now);
                    v
                }
                Owner::Macro(mi) => {
                    let (_, ti) = lane.mapped_by.expect("macro-owned lanes are mapped");
                    let mapped = self.macros[mi].targets[ti].value(macro_values[mi]);
                    match &lane.reclaim {
                        Some(r) => {
                            let k = r.blend.progress(now);
                            let v = interp(r.from, mapped, k, lane.log);
                            if r.blend.done(now) {
                                lane.reclaim = None;
                            }
                            v
                        }
                        None => mapped,
                    }
                }
            };
        }
    }

    /// Current value of each parameter, by id (as of the last [`tick`](Self::tick)).
    pub fn values(&self) -> impl Iterator<Item = f32> + '_ {
        self.lanes.iter().map(|l| l.out)
    }

    /// A report of everything that is moving, for the agent.
    pub fn state(&self, now: f64) -> State {
        let macros = self
            .macros
            .iter()
            .zip(&self.macro_lanes)
            .map(|(m, l)| MacroState {
                name: m.name.clone(),
                description: m.description.clone(),
                value: round(l.at(now)),
                target: round(l.target()),
                seconds_remaining: round(l.remaining(now)),
            })
            .collect();

        let params = self
            .params
            .iter()
            .zip(&self.lanes)
            .map(|(p, lane)| {
                let (owner, target, remaining) = match lane.owner {
                    Owner::Manual => (
                        "manual".to_string(),
                        lane.manual.target(),
                        lane.manual.remaining(now),
                    ),
                    Owner::Macro(mi) => {
                        let (_, ti) = lane.mapped_by.expect("mapped");
                        let target =
                            self.macros[mi].targets[ti].value(self.macro_lanes[mi].target());
                        let remaining = self.macro_lanes[mi].remaining(now).max(
                            lane.reclaim
                                .as_ref()
                                .map_or(0.0, |r| r.blend.remaining(now)),
                        );
                        (format!("macro:{}", self.macros[mi].name), target, remaining)
                    }
                };
                ParamState {
                    path: p.path.clone(),
                    value: round(lane.out),
                    target: round(target),
                    min: p.min,
                    max: p.max,
                    unit: p.unit.clone(),
                    owner,
                    seconds_remaining: round(remaining),
                }
            })
            .collect();

        State {
            pace: self.pace,
            macros,
            params,
        }
    }
}

fn check_duration(over: f32) -> Result<f32, DirectorError> {
    if !over.is_finite() || !(0.0..=MAX_DURATION).contains(&over) {
        return Err(DirectorError::Invalid(format!(
            "'over' must be between 0 and {MAX_DURATION} seconds"
        )));
    }
    Ok(over.max(MIN_DURATION))
}

fn round(x: f32) -> f32 {
    (x * 1000.0).round() / 1000.0
}

#[derive(Clone, Debug, Serialize)]
pub struct State {
    /// Default duration and curve for calls that don't give their own.
    pub pace: Pace,
    pub macros: Vec<MacroState>,
    pub params: Vec<ParamState>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MacroState {
    pub name: String,
    pub description: String,
    pub value: f32,
    pub target: f32,
    pub seconds_remaining: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParamState {
    pub path: String,
    pub value: f32,
    pub target: f32,
    pub min: f32,
    pub max: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// `"macro:<name>"` or `"manual"`.
    pub owner: String,
    pub seconds_remaining: f32,
}

/// Runs a [`Director`] against a live engine on a control thread.
///
/// Clone it freely: all clones share the same director and clock.
#[derive(Clone)]
pub struct Conductor {
    director: Arc<Mutex<Director>>,
    engine: EngineHandle,
    clock: Instant,
    running: Arc<AtomicBool>,
}

impl Conductor {
    /// Push the director's starting values to the engine and start the
    /// control loop. Call before audio starts sounding, if possible, so the
    /// first sound is already the right one.
    pub fn start(director: Director, engine: EngineHandle) -> Self {
        for (id, v) in director.values().enumerate() {
            engine.set_target(id, v);
        }
        let c = Self {
            director: Arc::new(Mutex::new(director)),
            engine,
            clock: Instant::now(),
            running: Arc::new(AtomicBool::new(true)),
        };
        let loop_c = c.clone();
        thread::Builder::new()
            .name("moodscore-director".into())
            .spawn(move || loop_c.run())
            .expect("failed to start the director thread");
        c
    }

    fn run(&self) {
        let period = Duration::from_secs_f64(1.0 / CONTROL_RATE_HZ as f64);
        while self.running.load(Ordering::Relaxed) {
            let now = self.now();
            {
                let mut d = self.lock();
                d.tick(now);
                for (id, v) in d.values().enumerate() {
                    self.engine.set_target(id, v);
                }
            }
            thread::sleep(period);
        }
    }

    /// Seconds since the conductor started.
    pub fn now(&self) -> f64 {
        self.clock.elapsed().as_secs_f64()
    }

    pub fn engine(&self) -> &EngineHandle {
        &self.engine
    }

    pub fn morph(
        &self,
        changes: &[(&str, f32)],
        over: f32,
        curve: Curve,
    ) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock().morph(now, changes, over, curve)
    }

    pub fn gesture(
        &self,
        path: &str,
        to: f32,
        over: f32,
        curve: Curve,
        start_in: f32,
    ) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock().gesture(now, path, to, over, curve, start_in)
    }

    pub fn state(&self) -> State {
        let now = self.now();
        self.lock().state(now)
    }

    pub fn pace(&self) -> Pace {
        self.lock().pace()
    }

    pub fn set_pace(&self, over: Option<f32>, curve: Option<Curve>) -> Result<Pace, DirectorError> {
        self.lock().set_pace(over, curve)
    }

    /// Stop the control loop.
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    fn lock(&self) -> MutexGuard<'_, Director> {
        // A panic while holding the lock leaves the director usable: recover.
        self.director.lock().unwrap_or_else(|e| e.into_inner())
    }
}
