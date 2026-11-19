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
//! Every parameter is either under macro control or manual control. Under
//! macro control, one macro may set it and others may add to it (see
//! [`mapping`]). A `gesture` takes a parameter into manual control, and it
//! stays there: a hand-set value is a deliberate choice, so later morphs
//! leave it alone. A morph with `reclaim` takes it back, gliding from
//! wherever it is to where the macros want it.
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
pub use mapping::{MacroDef, MappingError, Mode, DEFAULT_MAPPINGS};

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
    /// Driven by the macros in `ParamLane::mapped_by`.
    Macros,
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
    /// The macros that drive this parameter: (macro index, target index).
    mapped_by: Vec<(usize, usize)>,
    /// The parameter's value when no macro sets it (adds start from here).
    init: f32,
    min: f32,
    max: f32,
    out: f32,
    log: bool,
}

impl ParamLane {
    /// Where the macros put this parameter, given each macro's value.
    fn macro_value(&self, macros: &[MacroDef], macro_values: impl Fn(usize) -> f32) -> f32 {
        let mut base = self.init;
        let mut offset = 0.0;
        for &(mi, ti) in &self.mapped_by {
            let t = &macros[mi].targets[ti];
            let v = t.value(macro_values(mi));
            match t.mode {
                Mode::Set => base = v,
                Mode::Add => offset += v,
            }
        }
        (base + offset).clamp(self.min, self.max)
    }
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
                    mapped_by: Vec::new(),
                    init: p.init,
                    min: p.min,
                    max: p.max,
                    out: p.init,
                    log,
                }
            })
            .collect();

        for (mi, m) in macros.iter().enumerate() {
            for (ti, t) in m.targets.iter().enumerate() {
                let lane = &mut lanes[t.param];
                lane.mapped_by.push((mi, ti));
                lane.owner = Owner::Macros;
            }
        }
        for lane in &mut lanes {
            if lane.owner == Owner::Macros {
                lane.out = lane.macro_value(&macros, |mi| macros[mi].default);
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

    /// Move one or more mood macros. Parameters set by hand with
    /// [`gesture`](Self::gesture) stay where they are; see
    /// [`morph_with`](Self::morph_with) to take them back.
    pub fn morph(
        &mut self,
        now: f64,
        changes: &[(&str, f32)],
        over: f32,
        curve: Curve,
    ) -> Result<(), DirectorError> {
        self.morph_with(now, changes, over, curve, false)
    }

    /// Move one or more mood macros. With `reclaim`, parameters these macros
    /// drive that were set by hand come back under macro control, gliding
    /// from where they are over the same duration.
    pub fn morph_with(
        &mut self,
        now: f64,
        changes: &[(&str, f32)],
        over: f32,
        curve: Curve,
        reclaim: bool,
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
                if lane.owner == Owner::Manual && !reclaim {
                    continue;
                }
                if lane.owner != Owner::Macros || lane.reclaim.is_some() {
                    let from = lane.out;
                    lane.owner = Owner::Macros;
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
                Owner::Macros => {
                    let mapped = lane.macro_value(&self.macros, |mi| macro_values[mi]);
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
                    Owner::Macros => {
                        let target =
                            lane.macro_value(&self.macros, |mi| self.macro_lanes[mi].target());
                        let remaining = lane
                            .mapped_by
                            .iter()
                            .map(|&(mi, _)| self.macro_lanes[mi].remaining(now))
                            .fold(
                                lane.reclaim
                                    .as_ref()
                                    .map_or(0.0, |r| r.blend.remaining(now)),
                                f32::max,
                            );
                        let names: Vec<&str> = lane
                            .mapped_by
                            .iter()
                            .map(|&(mi, _)| self.macros[mi].name.as_str())
                            .collect();
                        (format!("macro:{}", names.join("+")), target, remaining)
                    }
                };
                let held_from = if lane.owner == Owner::Manual {
                    lane.mapped_by
                        .iter()
                        .map(|&(mi, _)| self.macros[mi].name.clone())
                        .collect()
                } else {
                    Vec::new()
                };
                ParamState {
                    held_from,
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
    /// `"manual"`, or `"macro:<name>"` (`"macro:<a>+<b>"` when several macros drive it).
    pub owner: String,
    pub seconds_remaining: f32,
    /// For a hand-set parameter, the macros that would drive it but are
    /// leaving it alone (until a morph with `reclaim`).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub held_from: Vec<String>,
}

/// A transport fade waiting to start.
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
pub struct ScheduledFade {
    /// Output gain the fade goes to: 1 for a start, 0 for a release.
    pub to: f32,
    /// Seconds until it starts.
    pub starts_in: f32,
    /// Seconds the fade takes once it starts.
    pub over: f32,
}

#[derive(Copy, Clone, Debug)]
struct PendingFade {
    at: f64,
    to: f32,
    over: f32,
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
    pending_fade: Arc<Mutex<Option<PendingFade>>>,
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
            pending_fade: Arc::new(Mutex::new(None)),
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
            {
                let mut pending = self.pending();
                if let Some(f) = *pending {
                    if now >= f.at {
                        self.engine.fade_to(f.to, f.over);
                        *pending = None;
                    }
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
        reclaim: bool,
    ) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock().morph_with(now, changes, over, curve, reclaim)
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

    /// Fade the output to `to` (0 = silence, 1 = full) over `over` seconds,
    /// starting in `start_in` seconds. Replaces any fade still waiting to
    /// start; a fade already under way is taken over from where it is.
    pub fn fade(&self, to: f32, over: f32, start_in: f32) {
        let mut pending = self.pending();
        if start_in > 0.0 {
            *pending = Some(PendingFade {
                at: self.now() + start_in as f64,
                to,
                over,
            });
        } else {
            *pending = None;
            self.engine.fade_to(to, over);
        }
    }

    /// The transport fade waiting to start, if any.
    pub fn scheduled_fade(&self) -> Option<ScheduledFade> {
        let now = self.now();
        self.pending().map(|f| ScheduledFade {
            to: f.to,
            starts_in: round((f.at - now).max(0.0) as f32),
            over: f.over,
        })
    }

    fn pending(&self) -> MutexGuard<'_, Option<PendingFade>> {
        self.pending_fade.lock().unwrap_or_else(|e| e.into_inner())
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
