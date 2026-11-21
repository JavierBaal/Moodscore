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

pub mod cues;
pub mod curve;
pub mod harmony;
pub mod mapping;
pub mod sequence;
pub mod tempo;

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::{Duration, Instant};

use moodscore_engine::{EngineHandle, NoteEvent, NoteMsg, ParamInfo, Scale};
use serde::Serialize;

pub use curve::Curve;
pub use mapping::{MacroDef, MappingError, Mode, DEFAULT_MAPPINGS};
pub use tempo::{Meter, TempoMap};

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
    /// A count or a choice: nothing lies between values, so it switches
    /// when its gesture ends instead of gliding.
    step: bool,
}

impl Lane {
    fn new(value: f32, log: bool) -> Self {
        Self {
            value,
            gesture: None,
            log,
            step: false,
        }
    }

    fn stepped(mut self, step: bool) -> Self {
        self.step = step;
        self
    }

    fn at(&self, now: f64) -> f32 {
        match &self.gesture {
            Some(g) if self.step => {
                if g.done(now) {
                    g.to
                } else {
                    g.from
                }
            }
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
    /// A voice pitch driven by the harmonic plan (moves on `manual`).
    Harmony,
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
    /// Set by hand with `gesture` (as opposed to never touched).
    hand_set: bool,
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
#[derive(Clone)]
pub struct Director {
    params: Vec<ParamInfo>,
    macros: Vec<MacroDef>,
    macro_lanes: Vec<Lane>,
    lanes: Vec<ParamLane>,
    pace: Pace,
    /// Parameter ids of each voice's pitch: drone_1/pitch, drone_2/pitch, …
    voice_pitches: Vec<usize>,
    harmony: Option<HarmonyRun>,
    /// A plan waiting for its first beat while `harmony` walks on.
    next_harmony: Option<HarmonyRun>,
    /// Counts harmonic plans, so a new plan's chords are new chords.
    plans: u64,
    /// The chord last voiced, for the sequencer when no plan is walking.
    sounding: Option<sequence::ChordAt>,
    tempo: TempoMap,
}

/// How long a chord lasts: in seconds, or in musical time that follows the
/// tempo (and its ramps).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Length {
    Seconds(f32),
    /// Beats of the metre (quarters in 4/4, eighths in 6/8).
    Beats(f32),
    Bars(f32),
}

/// Where a new plan's first chord lands.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum StartOn {
    /// Right away.
    #[default]
    Now,
    /// On the next beat.
    Beat,
    /// On the next downbeat.
    Bar,
    /// On the downbeat of this bar (it must not have passed).
    AtBar(i64),
    /// At this quarter-note position on the tempo map (see
    /// [`TempoMap::beat_at`]); it must not have passed. What a cue list
    /// uses: beats never move, whatever the tempo does meanwhile.
    AtQuarter(f64),
    /// At this time on the director's clock (it must not have passed).
    AtTime(f64),
}

/// A harmonic plan: a key and a progression the director walks through.
#[derive(Clone, Debug)]
pub struct HarmonyPlan {
    pub key: harmony::Key,
    /// Each chord with how long it lasts.
    pub progression: Vec<(harmony::Chord, Length)>,
    pub looping: bool,
    /// A note held in the bass under every chord.
    pub pedal: Option<harmony::Note>,
    /// Seconds each voice takes to glide to its next note.
    pub glide: f32,
    pub curve: Curve,
    pub start_on: StartOn,
}

/// A plan being walked. Chords are counted from the start, laps included,
/// so the same chord on the next lap is a new step.
#[derive(Clone, Debug)]
struct HarmonyRun {
    plan: HarmonyPlan,
    /// Which plan this is, so its chords are told apart from another's.
    serial: u64,
    /// When the first chord lands.
    starts_at: f64,
    step: usize,
    /// When the current step began, in seconds and in quarter notes.
    step_time: f64,
    step_beat: f64,
    /// The step last voiced.
    applied: Option<usize>,
}

impl HarmonyRun {
    fn len(&self) -> usize {
        self.plan.progression.len()
    }

    /// A non-looping plan holds its last chord for ever.
    fn holding(&self) -> bool {
        !self.plan.looping && self.step + 1 >= self.len()
    }

    /// Quarter notes the current step lasts, if it is counted in musical time.
    fn step_quarters(&self, tempo: &TempoMap) -> Option<f64> {
        let meter = tempo.meter();
        match self.plan.progression[self.step % self.len()].1 {
            Length::Seconds(_) => None,
            Length::Beats(b) => Some(b as f64 * meter.quarters_per_beat()),
            Length::Bars(b) => Some(b as f64 * meter.quarters_per_bar()),
        }
    }

    /// When the current step ends, under `tempo`.
    fn step_end(&self, tempo: &TempoMap) -> f64 {
        if self.holding() {
            return f64::INFINITY;
        }
        match self.plan.progression[self.step % self.len()].1 {
            Length::Seconds(s) => self.step_time + s as f64,
            _ => tempo.time_at(self.step_beat + self.step_quarters(tempo).unwrap_or(0.0)),
        }
    }

    /// Move on through every step that has ended by `now`. Each step starts
    /// exactly when the previous one was due, so nothing drifts.
    fn walk(&mut self, now: f64, tempo: &TempoMap) {
        // Bounded: chords last at least a fraction of a second.
        for _ in 0..100_000 {
            let end = self.step_end(tempo);
            if now < end {
                return;
            }
            self.step_beat = match self.step_quarters(tempo) {
                Some(q) => self.step_beat + q,
                None => tempo.beat_at(end),
            };
            self.step_time = end;
            self.step += 1;
        }
    }
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
                    manual: Lane::new(p.init, log).stepped(p.scale == Scale::Discrete),
                    reclaim: None,
                    mapped_by: Vec::new(),
                    init: p.init,
                    min: p.min,
                    max: p.max,
                    out: p.init,
                    log,
                    hand_set: false,
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
            voice_pitches: Vec::new(),
            harmony: None,
            next_harmony: None,
            plans: 0,
            sounding: None,
            tempo: TempoMap::default(),
        };
        d.voice_pitches = (1..)
            .map_while(|v| {
                d.params
                    .iter()
                    .position(|p| p.path == format!("drone_{v}/pitch"))
            })
            .collect();
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
            lane.manual = Lane::new(lane.out, lane.log).stepped(lane.manual.step);
            lane.owner = Owner::Manual;
            lane.reclaim = None;
        }
        lane.hand_set = true;
        // A count or a choice switches at once (after start_in).
        let duration = if lane.manual.step {
            MIN_DURATION
        } else {
            duration
        };
        lane.manual.start(now, to, start_in, duration, curve);
        Ok(())
    }

    /// Start walking a harmonic plan, from its first chord, when its
    /// `start_on` says. Voice pitches nobody has set by hand follow the
    /// harmony; hand-set ones stay where they are unless `reclaim`. A plan
    /// starting now replaces the one walking; one that waits for a later
    /// beat lets the current plan walk on until then (replacing any other
    /// plan waiting).
    pub fn harmony(
        &mut self,
        now: f64,
        plan: HarmonyPlan,
        reclaim: bool,
    ) -> Result<(), DirectorError> {
        if self.voice_pitches.is_empty() {
            return Err(DirectorError::Invalid(
                "this instrument has no drone voices to harmonise".into(),
            ));
        }
        if plan.progression.is_empty() {
            return Err(DirectorError::Invalid(
                "the progression needs at least one chord".into(),
            ));
        }
        for &id in &self.voice_pitches {
            let lane = &mut self.lanes[id];
            let free = lane.owner == Owner::Manual && !lane.hand_set;
            if lane.owner == Owner::Harmony || free || reclaim {
                if lane.owner != Owner::Harmony {
                    lane.manual = Lane::new(lane.out, lane.log).stepped(lane.manual.step);
                    lane.reclaim = None;
                }
                lane.owner = Owner::Harmony;
                lane.hand_set = false;
            }
        }
        let starts_at = self.start_time(now, plan.start_on)?;
        self.plans += 1;
        let run = HarmonyRun {
            plan,
            serial: self.plans,
            starts_at,
            step: 0,
            step_time: starts_at,
            step_beat: self.tempo.beat_at(starts_at),
            applied: None,
        };
        let walking = self.harmony.as_ref().is_some_and(|r| now >= r.starts_at);
        if starts_at > now + 1e-6 && walking {
            self.next_harmony = Some(run);
        } else {
            self.harmony = Some(run);
            self.next_harmony = None;
        }
        self.advance_harmony(now);
        Ok(())
    }

    /// The tempo map in force.
    pub fn tempo(&self) -> &TempoMap {
        &self.tempo
    }

    /// The chord sounding at time `t` (now or later), as far as the plan
    /// walking now says: the chord last voiced while a new plan waits for
    /// its first beat, or after the harmony stopped. None if no harmony has
    /// played yet.
    pub fn chord_at(&self, t: f64) -> Option<sequence::ChordAt> {
        for run in [&self.next_harmony, &self.harmony].into_iter().flatten() {
            if t >= run.starts_at {
                let mut r = run.clone();
                r.walk(t, &self.tempo);
                return Some(sequence::ChordAt {
                    chord: r.plan.progression[r.step % r.len()].0.clone(),
                    key: r.plan.key.clone(),
                    id: r.serial << 32 | r.step as u64,
                });
            }
        }
        self.sounding.clone()
    }

    /// When something asked to start `on` would start, seen from `now`.
    pub fn start_time(&self, now: f64, on: StartOn) -> Result<f64, DirectorError> {
        Ok(match on {
            StartOn::Now => now,
            StartOn::Beat => self.tempo.next_beat(now),
            StartOn::Bar => self.tempo.next_bar(now),
            StartOn::AtQuarter(_) | StartOn::AtTime(_) => {
                let t = match on {
                    StartOn::AtQuarter(q) => self.tempo.time_at(q),
                    StartOn::AtTime(t) => t,
                    _ => unreachable!(),
                };
                if t < now - 0.005 {
                    return Err(DirectorError::Invalid(format!(
                        "that moment passed {:.2} s ago",
                        now - t
                    )));
                }
                t.max(now)
            }
            StartOn::AtBar(n) => {
                let t = self.tempo.time_of_bar(n);
                // A few milliseconds of slack: "this bar" said right on it.
                if t < now - 0.005 {
                    let current = self.tempo.position(now).bar;
                    return Err(DirectorError::Invalid(format!(
                        "bar {n} has passed; it is bar {current} now"
                    )));
                }
                if t - now > MAX_DURATION as f64 {
                    return Err(DirectorError::Invalid(format!(
                        "bar {n} is more than {MAX_DURATION} seconds away"
                    )));
                }
                t.max(now)
            }
        })
    }

    /// Change tempo and/or metre at `now`. See [`TempoMap::changed`]: the beat
    /// carries on unbroken, a new metre starts a new bar, and with a ramp the
    /// tempo glides to `bpm` over `ramp` seconds. Chords counted in beats or
    /// bars stretch or shrink with it.
    pub fn set_tempo(
        &mut self,
        now: f64,
        bpm: Option<f64>,
        meter: Option<Meter>,
        ramp: f64,
        curve: Curve,
        downbeat: bool,
    ) -> Result<(), DirectorError> {
        if let Some(b) = bpm {
            if !b.is_finite() || !(tempo::MIN_BPM..=tempo::MAX_BPM).contains(&b) {
                return Err(DirectorError::Invalid(format!(
                    "bpm must be between {} and {}",
                    tempo::MIN_BPM,
                    tempo::MAX_BPM
                )));
            }
        }
        if !ramp.is_finite() || !(0.0..=MAX_DURATION as f64).contains(&ramp) {
            return Err(DirectorError::Invalid(format!(
                "a tempo change can take at most {MAX_DURATION} seconds"
            )));
        }
        // Settle the harmony under the old tempo first: whatever was due
        // before now happened at the old speed.
        self.advance_harmony(now);
        self.tempo = self.tempo.changed(now, bpm, meter, ramp, curve, downbeat);
        Ok(())
    }

    /// Change tempo and/or metre when `on` says (now, or a beat ahead): a
    /// change ahead waits on the tempo map, so the harmony, the sequences and
    /// phrases worked out before it happens already follow it. `ramp` in
    /// quarter notes covers exactly that many. Returns when it happens.
    #[allow(clippy::too_many_arguments)]
    pub fn tempo_at(
        &mut self,
        now: f64,
        on: StartOn,
        bpm: Option<f64>,
        meter: Option<Meter>,
        ramp: tempo::Ramp,
        curve: Curve,
        downbeat: bool,
    ) -> Result<f64, DirectorError> {
        let at = self.start_time(now, on)?;
        let seconds = match ramp {
            tempo::Ramp::Seconds(s) => s,
            tempo::Ramp::Quarters(q) => match bpm {
                Some(b) => self.tempo.ramp_seconds_for(at, b, meter, q, curve),
                None => 0.0,
            },
        };
        if at <= now + 1e-6 {
            self.set_tempo(now, bpm, meter, seconds, curve, downbeat)?;
            return Ok(now);
        }
        // Validate as a change now would, then let it wait.
        self.clone()
            .set_tempo(now, bpm, meter, seconds, curve, downbeat)?;
        let q = self.tempo.beat_at(at);
        self.tempo = self
            .tempo
            .scheduled(now, q, bpm, meter, ramp, curve, downbeat)
            .map_err(DirectorError::Invalid)?;
        Ok(at)
    }

    /// Stop walking the harmony. Voices stay on the notes they have (or are
    /// gliding to) and become free for `gesture` or a later plan.
    pub fn stop_harmony(&mut self) {
        self.harmony = None;
        self.next_harmony = None;
        for &id in &self.voice_pitches {
            let lane = &mut self.lanes[id];
            if lane.owner == Owner::Harmony {
                lane.owner = Owner::Manual;
                lane.hand_set = false;
            }
        }
    }

    /// Voice the chord that should be sounding at `now`, if it changed.
    fn advance_harmony(&mut self, now: f64) {
        if self
            .next_harmony
            .as_ref()
            .is_some_and(|n| now >= n.starts_at)
        {
            self.harmony = self.next_harmony.take();
        }
        let tempo = self.tempo;
        let Some(run) = &mut self.harmony else { return };
        if now < run.starts_at {
            return;
        }
        run.walk(now, &tempo);
        let step = run.step;
        if run.applied == Some(step) {
            return;
        }
        // Start the change when the chord was due, not when we noticed: the
        // control loop runs at a fixed rate, and offline renders may jump.
        let at = run.step_time;
        let idx = step % run.len();
        let chord = run.plan.progression[idx].0.clone();
        self.sounding = Some(sequence::ChordAt {
            chord: chord.clone(),
            key: run.plan.key.clone(),
            id: run.serial << 32 | step as u64,
        });
        let (pedal, glide, curve) = (run.plan.pedal, run.plan.glide, run.plan.curve);

        let owned: Vec<usize> = self
            .voice_pitches
            .iter()
            .copied()
            .filter(|&id| self.lanes[id].owner == Owner::Harmony)
            .collect();
        let bass_id = self.voice_pitches[0];
        let bass_from = owned
            .contains(&bass_id)
            .then(|| harmony::hz_to_midi(self.lanes[bass_id].manual.target()));
        let upper_ids: Vec<usize> = owned.iter().copied().filter(|&id| id != bass_id).collect();
        // Voice-lead from where each voice is heading, so a chord change in
        // the middle of a glide still moves the least.
        let upper_from: Vec<f32> = upper_ids
            .iter()
            .map(|&id| harmony::hz_to_midi(self.lanes[id].manual.target()))
            .collect();
        let v = harmony::voice(&chord, pedal, bass_from, &upper_from);

        let duration = glide.max(MIN_DURATION);
        let mut moves: Vec<(usize, f32)> = upper_ids.into_iter().zip(v.upper).collect();
        if let Some(b) = v.bass {
            moves.push((bass_id, b));
        }
        for (id, midi) in moves {
            let p = &self.params[id];
            let hz = harmony::midi_to_hz(midi).clamp(p.min, p.max);
            self.lanes[id].manual.start(at, hz, 0.0, duration, curve);
        }
        if let Some(run) = &mut self.harmony {
            run.applied = Some(step);
        }
    }

    /// Advance to `now`. Read the result with [`values`](Self::values).
    pub fn tick(&mut self, now: f64) {
        self.tempo.advance(now);
        self.advance_harmony(now);
        let macro_values: Vec<f32> = self.macro_lanes.iter().map(|l| l.at(now)).collect();
        for l in &mut self.macro_lanes {
            l.settle(now);
        }

        for lane in &mut self.lanes {
            lane.out = match lane.owner {
                Owner::Manual | Owner::Harmony => {
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

    /// Where a parameter is heading: the end of its gesture if it is under
    /// manual or harmony control, its current value under the macros.
    pub fn target(&self, id: usize) -> Option<f32> {
        let lane = self.lanes.get(id)?;
        Some(match lane.owner {
            Owner::Manual | Owner::Harmony => lane.manual.target(),
            Owner::Macros => lane.out,
        })
    }

    /// Set several parameters by hand at once, each gliding over `over`
    /// seconds (counts and choices switch). All or nothing: if one is
    /// invalid, none changes.
    pub fn set_params(
        &mut self,
        now: f64,
        changes: &[(String, f32)],
        over: f32,
    ) -> Result<(), DirectorError> {
        let mut trial = self.clone();
        for (path, v) in changes {
            trial.gesture(now, path, *v, over, Curve::EaseInOut, 0.0)?;
        }
        *self = trial;
        Ok(())
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
            .enumerate()
            .map(|(id, (p, lane))| {
                let (owner, target, remaining) = match lane.owner {
                    Owner::Manual => (
                        "manual".to_string(),
                        lane.manual.target(),
                        lane.manual.remaining(now),
                    ),
                    Owner::Harmony => (
                        "harmony".to_string(),
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
                    let mut from: Vec<String> = lane
                        .mapped_by
                        .iter()
                        .map(|&(mi, _)| self.macros[mi].name.clone())
                        .collect();
                    if lane.hand_set && self.harmony.is_some() && self.voice_pitches.contains(&id) {
                        from.push("harmony".into());
                    }
                    from
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

        let pos = self.tempo.position(now);
        let ramp = self.tempo.ramp_at(now);
        State {
            pace: self.pace,
            tempo: TempoState {
                bpm: round1(self.tempo.bpm_at(now)),
                meter: self.tempo.meter().to_string(),
                bar: pos.bar,
                beat: (pos.beat * 100.0).floor() / 100.0,
                seconds_per_bar: round(self.tempo.seconds_per_bar(now) as f32),
                ramping_to: ramp.map(|(b, _)| round1(b)),
                ramp_seconds_remaining: ramp.map(|(_, s)| round(s as f32)),
                changes_ahead: self
                    .tempo
                    .pending_changes()
                    .into_iter()
                    .map(|c| TempoChangeState {
                        // The bar it lands in, by today's count (on a
                        // downbeat, the bar it opens).
                        bar: self.tempo.position(c.time - 1e-9).bar
                            + i64::from(self.tempo.next_bar(c.time - 1e-6) <= c.time + 1e-6),
                        in_seconds: round((c.time - now).max(0.0) as f32),
                        bpm: c.bpm.map(round1),
                        meter: c.meter.map(|m| m.to_string()),
                    })
                    .collect(),
            },
            harmony: self.harmony_state(now),
            macros,
            params,
        }
    }

    fn harmony_state(&self, now: f64) -> Option<HarmonyState> {
        let mut run = self.harmony.clone()?;
        let waiting = now < run.starts_at;
        if !waiting {
            run.walk(now, &self.tempo);
        }
        let plan = &run.plan;
        let n = plan.progression.len();
        let idx = run.step % n;
        let chord = &plan.progression[idx].0;
        let next = (!run.holding()).then(|| &plan.progression[(idx + 1) % n].0);
        let chord_state = |c: &harmony::Chord| ChordState {
            symbol: c.symbol.clone(),
            name: c.name.clone(),
            notes: c.note_names(),
        };
        Some(HarmonyState {
            key: format!("{} {}", plan.key.name(plan.key.tonic), plan.key.mode),
            starts_in: waiting.then(|| round((run.starts_at - now) as f32)),
            chord: chord_state(chord),
            next: if waiting { None } else { next.map(chord_state) },
            seconds_to_next: if waiting {
                None
            } else {
                next.map(|_| round((run.step_end(&self.tempo) - now).max(0.0) as f32))
            },
            position: format!("{}/{}", idx + 1, n),
            looping: plan.looping,
            pedal: plan.pedal.map(|p| plan.key.name(p.pitch_class).to_string()),
            then: self.next_harmony.as_ref().map(|n| NextPlanState {
                key: format!("{} {}", n.plan.key.name(n.plan.key.tonic), n.plan.key.mode),
                starts_in: round((n.starts_at - now).max(0.0) as f32),
                chord: chord_state(&n.plan.progression[0].0),
            }),
        })
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

fn round1(x: f64) -> f32 {
    ((x * 10.0).round() / 10.0) as f32
}

#[derive(Clone, Debug, Serialize)]
pub struct State {
    /// Default duration and curve for calls that don't give their own.
    pub pace: Pace,
    pub tempo: TempoState,
    /// The harmonic plan being walked, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmony: Option<HarmonyState>,
    pub macros: Vec<MacroState>,
    pub params: Vec<ParamState>,
}

/// The musical clock.
#[derive(Clone, Debug, Serialize)]
pub struct TempoState {
    /// Beats per minute, in the metre's beat unit.
    pub bpm: f32,
    pub meter: String,
    /// Bar 1, beat 1 is the first downbeat.
    pub bar: i64,
    pub beat: f64,
    pub seconds_per_bar: f32,
    /// The tempo a ramp under way is heading for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramping_to: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ramp_seconds_remaining: Option<f32>,
    /// Changes waiting for their beat: when (bar and seconds) and to what.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub changes_ahead: Vec<TempoChangeState>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TempoChangeState {
    pub bar: i64,
    pub in_seconds: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpm: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct HarmonyState {
    /// e.g. "A aeolian".
    pub key: String,
    /// Seconds until the first chord lands, for a plan waiting for its beat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_in: Option<f32>,
    pub chord: ChordState,
    /// The chord coming next, unless the plan has ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<ChordState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_next: Option<f32>,
    /// Which chord of the progression, e.g. "2/4".
    pub position: String,
    #[serde(rename = "loop")]
    pub looping: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pedal: Option<String>,
    /// A plan waiting for its first beat while this one walks on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<NextPlanState>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NextPlanState {
    pub key: String,
    pub starts_in: f32,
    pub chord: ChordState,
}

#[derive(Clone, Debug, Serialize)]
pub struct ChordState {
    /// As written in the plan: "VI".
    pub symbol: String,
    /// Readable name: "F".
    pub name: String,
    pub notes: Vec<String>,
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
    /// Notes waiting to be handed to the engine, by frame. They go when they
    /// are less than [`NOTE_LOOKAHEAD`] away, so the audio thread only ever
    /// holds a few, and a stop can still cancel the rest.
    agenda: Arc<Mutex<Vec<Queued>>>,
    /// Sequences playing or waiting to start, oldest first.
    sequences: Arc<Mutex<Vec<Running>>>,
    /// How the director's clock maps to engine frames, for everything
    /// scheduled ahead: one line for every part, so layers that start
    /// together sound together.
    line: Arc<Mutex<Option<FrameLine>>>,
    /// The cue list running, if any.
    cues: Arc<Mutex<Option<cues::Running>>>,
    /// The last [`HEARD_SECONDS`] of output, interleaved stereo, for `listen`.
    heard: Arc<Mutex<std::collections::VecDeque<f32>>>,
}

/// Seconds of output kept for listening back.
pub const HEARD_SECONDS: usize = 30;

/// How far ahead notes are handed to the engine.
const NOTE_LOOKAHEAD: f64 = 0.2;

/// How far ahead sequences are worked out into the agenda.
const SEQUENCE_AHEAD: f64 = 0.5;

/// Headroom so nothing scheduled ahead reaches the engine late.
const MARGIN: f64 = 0.03;

/// A note in the agenda, and the sequence it came from (0 for a phrase).
#[derive(Copy, Clone, Debug)]
struct Queued {
    msg: NoteMsg,
    seq: u64,
}

/// A sequence a part is playing or will play.
struct Running {
    id: u64,
    part: usize,
    /// What it plays, in words, for the reports.
    what: String,
    runner: sequence::Runner,
}

/// A straight line from the director's clock to engine frames.
#[derive(Copy, Clone, Debug)]
struct FrameLine {
    t0: f64,
    f0: f64,
}

/// What a part's sequence is doing, for the reports.
#[derive(Clone, Debug, Serialize)]
pub struct SequenceState {
    /// What it plays, e.g. "arpeggio up_down F4-C#5 in 1/8t".
    pub plays: String,
    /// Seconds until it starts, if it is waiting for its bar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_in: Option<f64>,
    /// Seconds until it stops, if it is due to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_in: Option<f64>,
}

/// Most notes waiting in the agenda.
pub const MAX_AGENDA: usize = 20_000;

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
            agenda: Arc::new(Mutex::new(Vec::new())),
            sequences: Arc::new(Mutex::new(Vec::new())),
            line: Arc::new(Mutex::new(None)),
            cues: Arc::new(Mutex::new(None)),
            heard: Arc::new(Mutex::new(std::collections::VecDeque::new())),
        };
        let loop_c = c.clone();
        let capture = c.engine.take_capture();
        thread::Builder::new()
            .name("moodscore-director".into())
            .spawn(move || loop_c.run(capture))
            .expect("failed to start the director thread");
        c
    }

    fn run(&self, mut capture: Option<rtrb::Consumer<f32>>) {
        let period = Duration::from_secs_f64(1.0 / CONTROL_RATE_HZ as f64);
        let keep = HEARD_SECONDS * self.engine.sample_rate() as usize * 2;
        while self.running.load(Ordering::Relaxed) {
            if let Some(cap) = &mut capture {
                let mut heard = self.heard.lock().unwrap_or_else(|e| e.into_inner());
                while let Ok(s) = cap.pop() {
                    heard.push_back(s);
                }
                let extra = heard.len().saturating_sub(keep);
                // Whole frames only, so left and right stay paired.
                heard.drain(..extra - extra % 2);
            }
            let now = self.now();
            {
                let mut d = self.lock();
                d.tick(now);
                for (id, v) in d.values().enumerate() {
                    self.engine.set_target(id, v);
                }
                self.engine
                    .set_tempo(d.tempo().quarters_per_second(now) as f32);
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
            self.feed_cues(now);
            self.feed_sequences(now);
            self.feed_notes();
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

    pub fn harmony(&self, plan: HarmonyPlan, reclaim: bool) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock().harmony(now, plan, reclaim)
    }

    pub fn stop_harmony(&self) {
        self.lock().stop_harmony()
    }

    pub fn set_tempo(
        &self,
        bpm: Option<f64>,
        meter: Option<Meter>,
        ramp: f64,
        curve: Curve,
        downbeat: bool,
    ) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock()
            .set_tempo(now, bpm, meter, ramp, curve, downbeat)
    }

    /// The tempo map in force.
    pub fn tempo(&self) -> TempoMap {
        *self.lock().tempo()
    }

    /// Do something with the director itself, holding it for that long
    /// (never call back into the conductor from `f`).
    pub fn with_director<R>(&self, f: impl FnOnce(&mut Director) -> R) -> R {
        f(&mut self.lock())
    }

    /// Run a cue list: its first section begins when something started
    /// `on` would, and the conductor plays the rest on its own, each action
    /// at its moment. Replaces the list running: what it had not done yet
    /// is dropped; what it set going plays on.
    pub fn cue(&self, list: cues::CueList, on: StartOn) -> Result<cues::CueState, DirectorError> {
        if list.sections.is_empty() {
            return Err(DirectorError::Invalid("there are no sections".into()));
        }
        let start = self.start_time(on)?;
        let tempo = self.tempo();
        let running = cues::Running::new(list, cues::Point::Quarter(tempo.beat_at(start)), &tempo);
        *self.cues() = Some(running);
        let now = self.now();
        self.feed_cues(now);
        Ok(self.cue_state().expect("just set"))
    }

    /// Stop the cue list: nothing more of it happens; what it set going
    /// plays on.
    pub fn stop_cues(&self) {
        *self.cues() = None;
    }

    /// What the cue list is doing, if one runs.
    pub fn cue_state(&self) -> Option<cues::CueState> {
        let cues = self.cues();
        let r = cues.as_ref()?;
        let now = self.now();
        Some(r.state(now, &self.tempo()))
    }

    fn cues(&self) -> MutexGuard<'_, Option<cues::Running>> {
        self.cues.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Run whatever the cue list has due.
    fn feed_cues(&self, now: f64) {
        let runs = {
            let mut cues = self.cues();
            let Some(r) = cues.as_mut() else { return };
            let tempo = self.tempo();
            let runs = r.due(now, &tempo);
            if r.finished(now, &tempo) {
                *cues = None;
            }
            runs
        };
        for (run, on, what) in runs {
            if let Err(e) = run(self, now, on) {
                if let Some(r) = self.cues().as_mut() {
                    r.failed(format!("{what}: {e}"));
                }
            }
        }
    }

    /// A copy of the director as it is now, and the time on its clock: to
    /// work out ahead what would happen, offline, without touching it.
    pub fn snapshot(&self) -> (Director, f64) {
        let d = self.lock();
        (d.clone(), self.now())
    }

    /// When something asked to start `on` would start (see
    /// [`Director::start_time`]), on this clock.
    pub fn start_time(&self, on: StartOn) -> Result<f64, DirectorError> {
        let now = self.now();
        self.lock().start_time(now, on)
    }

    /// Set several parameters by hand at once (see [`Director::set_params`]).
    pub fn set_params(&self, changes: &[(String, f32)], over: f32) -> Result<(), DirectorError> {
        let now = self.now();
        self.lock().set_params(now, changes, over)
    }

    /// Where each parameter in `ids` is heading.
    pub fn targets(&self, ids: impl IntoIterator<Item = usize>) -> Vec<f32> {
        let d = self.lock();
        ids.into_iter()
            .map(|id| d.target(id).unwrap_or(0.0))
            .collect()
    }

    fn agenda(&self) -> MutexGuard<'_, Vec<Queued>> {
        self.agenda.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Play note events on `part`, each at a time on the director's clock
    /// (see [`now`](Self::now)). Times already past play at once. Each is
    /// fixed to an engine frame now, so a phrase keeps its exact timing,
    /// and starts on that sample, a few milliseconds after the time asked
    /// for so that none arrives late. Fails, queueing nothing, if the
    /// agenda would overflow.
    pub fn schedule_notes(
        &self,
        part: usize,
        events: &[(f64, NoteEvent)],
    ) -> Result<(), DirectorError> {
        self.queue(part, 0, events)?;
        self.feed_notes();
        Ok(())
    }

    /// Put events in the agenda, fixed to frames on the shared line.
    fn queue(
        &self,
        part: usize,
        seq: u64,
        events: &[(f64, NoteEvent)],
    ) -> Result<(), DirectorError> {
        let line = self.frame_line();
        let mut agenda = self.agenda();
        if agenda.len() + events.len() > MAX_AGENDA {
            return Err(DirectorError::Invalid(format!(
                "{} notes are already waiting; at most {MAX_AGENDA}",
                agenda.len()
            )));
        }
        for &(t, event) in events {
            agenda.push(Queued {
                msg: NoteMsg {
                    part,
                    frame: line(t),
                    event,
                },
                seq,
            });
        }
        // Stable: events at the same frame keep their order.
        agenda.sort_by_key(|m| m.msg.frame);
        Ok(())
    }

    /// The frame a time on the director's clock lands on. The line is set
    /// the first time and follows the audio from then on: forward at once
    /// if the audio gets ahead of it (nothing may be late), back only if
    /// the audio falls well behind (it stalled or restarted), so the beat
    /// stays steady.
    fn frame_line(&self) -> impl Fn(f64) -> u64 {
        let now = self.now();
        let sr = self.engine.sample_rate() as f64;
        let rendered = self.engine.frames_rendered() as f64;
        let mut line = self.line.lock().unwrap_or_else(|e| e.into_inner());
        let l = match *line {
            Some(l) => {
                let expected = l.f0 + (now - l.t0) * sr;
                if rendered > expected {
                    FrameLine {
                        t0: l.t0,
                        f0: l.f0 + rendered - expected,
                    }
                } else if expected - rendered > 0.1 * sr {
                    FrameLine {
                        t0: now,
                        f0: rendered,
                    }
                } else {
                    l
                }
            }
            None => FrameLine {
                t0: now,
                f0: rendered,
            },
        };
        *line = Some(l);
        move |t: f64| (l.f0 + (t - l.t0 + MARGIN) * sr).max(rendered) as u64
    }

    /// Start a sequence on `part` (see [`sequence`]): step 0 lands when
    /// something started `on` would (see [`start_time`](Self::start_time)),
    /// and it plays for `bars` bars, or until stopped. A sequence already
    /// on the part stops there, so a change lands on the beat. `plays` says
    /// what it plays, in words, for the reports.
    pub fn sequence(
        &self,
        part: usize,
        pattern: sequence::Pattern,
        plays: String,
        on: StartOn,
        bars: Option<f64>,
    ) -> Result<SequenceState, DirectorError> {
        let mut seqs = self.sequences();
        let now = self.now();
        let (start, origin, end) = {
            let d = self.lock();
            if pattern.needs_harmony() && d.harmony.is_none() && d.sounding.is_none() {
                return Err(DirectorError::Invalid(
                    "no harmony is playing to follow: start one with harmony, or give the \
                     arpeggio its notes, or write the motif in note names (F4) instead of degrees"
                        .into(),
                ));
            }
            let start = d.start_time(now, on)?;
            let origin = d.tempo.beat_at(start);
            let end = bars.map(|b| origin + b * d.tempo.meter().quarters_per_bar());
            (start, origin, end)
        };
        for s in seqs.iter_mut().filter(|s| s.part == part) {
            s.runner.stop_at(origin);
        }
        let ids: Vec<u64> = seqs
            .iter()
            .filter(|s| s.part == part)
            .map(|s| s.id)
            .collect();
        self.cut(&ids, start);
        let id = self.next_id();
        let mut runner =
            sequence::Runner::new(pattern, origin, id.wrapping_mul(0x9e37_79b9_7f4a_7c15));
        if let Some(e) = end {
            runner.stop_at(e);
        }
        seqs.push(Running {
            id,
            part,
            what: plays,
            runner,
        });
        let state = self.sequence_state(&seqs[seqs.len() - 1], now);
        drop(seqs);
        self.feed_sequences(now);
        Ok(state)
    }

    /// Stop `part`'s sequences when something started `on` would start
    /// (the next bar, say), letting go of their notes there.
    pub fn stop_sequence(&self, part: usize, on: StartOn) -> Result<(), DirectorError> {
        let mut seqs = self.sequences();
        let now = self.now();
        let (start, beat) = {
            let d = self.lock();
            let start = d.start_time(now, on)?;
            (start, d.tempo.beat_at(start))
        };
        for s in seqs.iter_mut().filter(|s| s.part == part) {
            s.runner.stop_at(beat);
        }
        let ids: Vec<u64> = seqs
            .iter()
            .filter(|s| s.part == part)
            .map(|s| s.id)
            .collect();
        self.cut(&ids, start);
        Ok(())
    }

    /// What `part`'s sequences are doing: the one playing, and the next if
    /// one waits for its bar.
    pub fn sequences_of(&self, part: usize) -> Vec<SequenceState> {
        let seqs = self.sequences();
        let now = self.now();
        seqs.iter()
            .filter(|s| s.part == part)
            .map(|s| self.sequence_state(s, now))
            .collect()
    }

    fn sequence_state(&self, s: &Running, now: f64) -> SequenceState {
        let tempo = self.tempo();
        let start = tempo.time_at(s.runner.origin());
        let secs = |t: f64| ((t - now).max(0.0) * 100.0).round() / 100.0;
        SequenceState {
            plays: s.what.clone(),
            starts_in: (start > now).then(|| secs(start)),
            ends_in: s.runner.end().map(|e| secs(tempo.time_at(e))),
        }
    }

    fn sequences(&self) -> MutexGuard<'_, Vec<Running>> {
        self.sequences.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn next_id(&self) -> u64 {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    /// Cut what the sequences `ids` already put in the agenda at time `at`:
    /// notes that would start from then on are dropped, and every note is
    /// let go by then.
    fn cut(&self, ids: &[u64], at: f64) {
        if ids.is_empty() {
            return;
        }
        let frame = self.frame_line()(at);
        let mut agenda = self.agenda();
        agenda.retain(|q| {
            !(ids.contains(&q.seq)
                && q.msg.frame >= frame
                && matches!(q.msg.event, NoteEvent::NoteOn { .. }))
        });
        for q in agenda.iter_mut() {
            if ids.contains(&q.seq) && q.msg.frame > frame {
                q.msg.frame = frame;
            }
        }
        agenda.sort_by_key(|m| m.msg.frame);
    }

    /// Work every sequence out a little ahead, into the agenda.
    fn feed_sequences(&self, now: f64) {
        let mut seqs = self.sequences();
        if seqs.is_empty() {
            return;
        }
        let mut batches = Vec::new();
        {
            let d = self.lock();
            let chords = |t: f64| d.chord_at(t);
            for s in seqs.iter_mut() {
                let events = s.runner.fill(now + SEQUENCE_AHEAD, &d.tempo, &chords);
                if !events.is_empty() {
                    batches.push((s.part, s.id, events));
                }
            }
            // A finished sequence is worked out ahead of time; it stays in
            // the reports until it has actually stopped.
            seqs.retain(|s| {
                !s.runner.done() || s.runner.end().is_some_and(|e| d.tempo.time_at(e) > now)
            });
        }
        drop(seqs);
        for (part, id, events) in batches {
            // A full agenda drops these steps rather than stalling the loop.
            let _ = self.queue(part, id, &events);
        }
    }

    /// The last `seconds` of output (at most [`HEARD_SECONDS`]), as left and
    /// right, at the engine's sample rate. Empty if nothing has been heard.
    pub fn recent_output(&self, seconds: f64) -> (Vec<f32>, Vec<f32>) {
        let heard = self.heard.lock().unwrap_or_else(|e| e.into_inner());
        let want = (seconds.max(0.0) * self.engine.sample_rate() as f64) as usize * 2;
        let skip = heard.len().saturating_sub(want);
        let skip = skip - skip % 2;
        let (mut l, mut r) = (Vec::new(), Vec::new());
        for (i, s) in heard.iter().skip(skip).enumerate() {
            if i % 2 == 0 {
                l.push(*s);
            } else {
                r.push(*s);
            }
        }
        r.truncate(l.len());
        l.truncate(r.len());
        (l, r)
    }

    /// Cancel every note still waiting for `part`, stop its sequences and
    /// release what sounds.
    pub fn stop_notes(&self, part: usize) {
        self.sequences().retain(|s| s.part != part);
        self.agenda().retain(|m| m.msg.part != part);
        let frame = self.engine.frames_rendered();
        self.engine.note(part, frame, NoteEvent::AllOff);
    }

    /// Notes of phrases waiting in the agenda for `part` (sequences aside),
    /// and the director time of the last one (when the phrase ends).
    pub fn pending_notes(&self, part: usize) -> (usize, Option<f64>) {
        let agenda = self.agenda();
        let mine: Vec<&NoteMsg> = agenda
            .iter()
            .filter(|m| m.msg.part == part && m.seq == 0)
            .map(|m| &m.msg)
            .collect();
        let last = mine.last().map(|m| {
            let ahead = m.frame as f64 - self.engine.frames_rendered() as f64;
            self.now() + ahead / self.engine.sample_rate() as f64
        });
        (mine.len(), last)
    }

    /// Hand the engine every note due within the lookahead.
    fn feed_notes(&self) {
        let horizon = self.engine.frames_rendered()
            + (NOTE_LOOKAHEAD * self.engine.sample_rate() as f64) as u64;
        let mut agenda = self.agenda();
        let due = agenda.partition_point(|m| m.msg.frame < horizon);
        let mut sent = 0;
        for q in &agenda[..due] {
            let m = q.msg;
            if !self.engine.note(m.part, m.frame, m.event) {
                break;
            }
            sent += 1;
        }
        agenda.drain(..sent);
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
