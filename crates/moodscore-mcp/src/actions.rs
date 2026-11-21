//! Every action an agent can take, validated once and run when its moment
//! comes: at once for a tool call, later for an action inside a section.
//!
//! Each `prepare_*` checks everything it can without changing anything (on
//! a copy of the director where the director has the last word) and returns
//! an [`Action`]: a closure that does it, called with the moment it lands
//! on, plus where a tool call of its own would land. A section is accepted
//! only if every one of its actions prepares, so nothing of a bad section
//! ever sounds.

use std::sync::{Arc, Mutex};

use moodscore_director::cues::CueRun;
use moodscore_director::harmony::{parse_note, Chord, Key};
use moodscore_director::tempo::Ramp;
use moodscore_director::{Conductor, Curve, HarmonyPlan, Meter, StartOn};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    check_fade, chord_length, curve_or, notes, start_on, start_on_bar, wait_for, GestureArgs,
    HarmonyArgs, Loaded, MorphArgs, PatchArgs, PlayNotesArgs, ReleaseArgs, SequenceArgs,
    SetPaceArgs, StartArgs, TempoArgs, DEFAULT_GLIDE, DEFAULT_RELEASE, DEFAULT_START, MAX_BARS,
    MAX_CHORDS, MAX_EVENTS, MAX_REPEAT, MAX_STEPS,
};

/// What kind of thing an action is: how far ahead of its moment it is set
/// up, and its turn among actions set up together.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Waits on the tempo map, so everything after knows it.
    Tempo,
    Pace,
    /// Waits for its beat while the current plan walks on.
    Harmony,
    /// Just before the bar, so the first note has the new sound.
    Patch,
    /// Morphs and gestures: on the moment.
    Move,
    /// Start and release: on the moment.
    Fade,
    /// Sequences and phrases: set up ahead, landing on the sample.
    Notes,
    /// Stopping something: on the moment.
    Stop,
}

impl Kind {
    pub fn lead(self) -> f64 {
        match self {
            Kind::Tempo | Kind::Harmony | Kind::Notes => 0.6,
            Kind::Patch => 0.05,
            Kind::Pace | Kind::Move | Kind::Fade | Kind::Stop => 0.0,
        }
    }

    pub fn order(self) -> u8 {
        match self {
            Kind::Tempo => 0,
            Kind::Pace => 1,
            Kind::Harmony => 2,
            Kind::Patch => 3,
            Kind::Move => 4,
            Kind::Fade => 5,
            Kind::Notes => 6,
            Kind::Stop => 7,
        }
    }
}

/// A validated action.
pub struct Action {
    pub kind: Kind,
    /// What it does, in words.
    pub what: String,
    pub run: CueRun,
    /// Where a tool call of its own lands (its start_on, at_bar, start_in).
    pub own: StartOn,
    /// How long it plays, in quarter notes, for a phrase.
    pub quarters: Option<f64>,
}

/// Where an action is being prepared.
pub struct Ctx<'a> {
    pub c: &'a Conductor,
    pub loaded: Arc<Mutex<Loaded>>,
    /// Inside a section: the section says when, so the action's own timing
    /// fields are refused.
    pub in_section: bool,
}

impl Ctx<'_> {
    /// Refuse timing fields inside a section.
    fn no_timing(&self, fields: &[(&str, bool)]) -> Result<(), String> {
        if !self.in_section {
            return Ok(());
        }
        match fields.iter().find(|(_, given)| *given) {
            Some((name, _)) => Err(format!(
                "inside a section the section says when: drop {name} (use after_bars to start later in the section)"
            )),
            None => Ok(()),
        }
    }
}

/// How long a move takes.
#[derive(Copy, Clone, Debug)]
enum Over {
    /// The pace in force when it runs.
    Pace,
    Seconds(f32),
    /// So many bars from where it starts, on the tempo map then.
    Bars(f64),
}

fn over(seconds: Option<f32>, bars: Option<f64>, max: f32) -> Result<Over, String> {
    match (seconds, bars) {
        (Some(_), Some(_)) => Err("give over or over_bars, not both".into()),
        (Some(s), None) => {
            if !s.is_finite() || !(0.0..=max).contains(&s) {
                return Err(format!("'over' must be between 0 and {max} seconds"));
            }
            Ok(Over::Seconds(s))
        }
        (None, Some(b)) => {
            if !b.is_finite() || !(0.0..=MAX_BARS).contains(&b) {
                return Err(format!("over_bars must be between 0 and {MAX_BARS}"));
            }
            Ok(Over::Bars(b))
        }
        (None, None) => Ok(Over::Pace),
    }
}

impl Over {
    /// Seconds, for a move starting at time `at`; `pace` when not given.
    fn seconds(self, c: &Conductor, at: f64, pace: f32) -> f32 {
        match self {
            Over::Pace => pace,
            Over::Seconds(s) => s,
            Over::Bars(b) => {
                let tempo = c.tempo();
                let q = tempo.beat_at(at);
                let per_bar = tempo.meter_at(at + 1e-6).quarters_per_bar();
                (tempo.time_at(q + b * per_bar) - at) as f32
            }
        }
    }
}

/// When `on` lands, and the seconds from `now` until then.
fn landing(c: &Conductor, now: f64, on: StartOn) -> Result<(f64, f32), String> {
    let at = c
        .with_director(|d| d.start_time(now, on))
        .map_err(|e| e.to_string())?;
    Ok((at, (at - now).max(0.0) as f32))
}

fn parse<T: DeserializeOwned>(tool: &str, v: Value) -> Result<T, String> {
    serde_json::from_value(v).map_err(|e| format!("{tool}: {e}"))
}

/// Prepare an action from its tool's name and arguments (inside a
/// section).
pub fn prepare(ctx: &Ctx, tool: &str, args: Value) -> Result<Action, String> {
    match tool {
        "start" => prepare_fade(ctx, parse::<StartArgs>(tool, args)?.into(), 1.0),
        "release" => prepare_fade(ctx, parse::<ReleaseArgs>(tool, args)?.into(), 0.0),
        "morph" => prepare_morph(ctx, parse(tool, args)?),
        "gesture" => prepare_gesture(ctx, parse(tool, args)?),
        "harmony" => prepare_harmony(ctx, parse(tool, args)?),
        "tempo" => prepare_tempo(ctx, parse(tool, args)?),
        "set_pace" => prepare_pace(ctx, parse(tool, args)?),
        "patch" => prepare_patch(ctx, parse(tool, args)?),
        "play_notes" => prepare_notes(ctx, parse(tool, args)?),
        "sequence" => prepare_sequence(ctx, parse(tool, args)?),
        other => Err(format!(
            "'{other}' can't go in a section; use harmony, tempo, morph, gesture, set_pace, patch, sequence, play_notes, start or release"
        )),
    }
}

/// The fields `start` and `release` share.
pub struct FadeArgs {
    pub over: Option<f32>,
    pub over_bars: Option<f64>,
    pub start_in: Option<f32>,
    pub at_bar: Option<i64>,
}

impl From<StartArgs> for FadeArgs {
    fn from(a: StartArgs) -> Self {
        FadeArgs {
            over: a.over,
            over_bars: a.over_bars,
            start_in: a.start_in,
            at_bar: a.at_bar,
        }
    }
}

impl From<ReleaseArgs> for FadeArgs {
    fn from(a: ReleaseArgs) -> Self {
        FadeArgs {
            over: a.over,
            over_bars: a.over_bars,
            start_in: a.start_in,
            at_bar: a.at_bar,
        }
    }
}

pub fn prepare_fade(ctx: &Ctx, a: FadeArgs, to: f32) -> Result<Action, String> {
    ctx.no_timing(&[
        ("start_in", a.start_in.is_some()),
        ("at_bar", a.at_bar.is_some()),
    ])?;
    let over = over(a.over, a.over_bars, crate::MAX_FADE)?;
    let default = if to > 0.0 {
        DEFAULT_START
    } else {
        DEFAULT_RELEASE
    };
    let own = if ctx.in_section {
        StartOn::Now
    } else {
        let wait = wait_for(ctx.c, a.start_in, a.at_bar)?;
        check_fade(0.0, wait)?;
        StartOn::AtTime(ctx.c.now() + wait as f64)
    };
    let what = if to > 0.0 { "start" } else { "release" };
    Ok(Action {
        kind: Kind::Fade,
        what: what.into(),
        own,
        quarters: None,
        run: Arc::new(move |c, now, on| {
            let (at, wait) = landing(c, now, on)?;
            let (over, wait) = check_fade(over.seconds(c, at, default), wait)?;
            c.fade(to, over, wait);
            Ok(())
        }),
    })
}

pub fn prepare_morph(ctx: &Ctx, a: MorphArgs) -> Result<Action, String> {
    let mut changes: Vec<(&'static str, f32)> = Vec::new();
    if let Some(v) = a.darkness {
        changes.push(("darkness", v));
    }
    if let Some(v) = a.instability {
        changes.push(("instability", v));
    }
    let over = over(a.over, a.over_bars, f32::MAX)?;
    let curve: Option<Curve> = a
        .curve
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(str::parse)
        .transpose()?;
    let reclaim = a.reclaim.unwrap_or(false);
    // Check on a copy: unknown macros, values out of range, durations.
    let (mut d, now) = ctx.c.snapshot();
    let pace = d.pace();
    let probe = match over {
        Over::Bars(_) => 1.0,
        o => o.seconds(ctx.c, now, pace.over),
    };
    d.morph_with(now, &changes, probe, curve.unwrap_or(pace.curve), reclaim)
        .map_err(|e| e.to_string())?;
    let what = changes
        .iter()
        .map(|(n, v)| format!("{n} {v}"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(Action {
        kind: Kind::Move,
        what: format!("morph {what}"),
        own: StartOn::Now,
        quarters: None,
        run: Arc::new(move |c, now, on| {
            let (at, _) = landing(c, now, on)?;
            let pace = c.pace();
            let over = over.seconds(c, at, pace.over);
            c.morph(&changes, over, curve.unwrap_or(pace.curve), reclaim)
                .map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_gesture(ctx: &Ctx, a: GestureArgs) -> Result<Action, String> {
    ctx.no_timing(&[("start_in", a.start_in.is_some())])?;
    let over = over(a.over, a.over_bars, f32::MAX)?;
    let curve: Option<Curve> = a
        .curve
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(str::parse)
        .transpose()?;
    let start_in = a.start_in.unwrap_or(0.0);
    if !start_in.is_finite() || !(0.0..=crate::MAX_FADE).contains(&start_in) {
        return Err(format!(
            "start_in must be between 0 and {} seconds",
            crate::MAX_FADE
        ));
    }
    let (mut d, now) = ctx.c.snapshot();
    let pace = d.pace();
    let probe = match over {
        Over::Bars(_) => 1.0,
        o => o.seconds(ctx.c, now, pace.over),
    };
    d.gesture(now, &a.param, a.to, probe, curve.unwrap_or(pace.curve), 0.0)
        .map_err(|e| e.to_string())?;
    let (param, to) = (a.param, a.to);
    Ok(Action {
        kind: Kind::Move,
        what: format!("gesture {param} to {to}"),
        own: StartOn::AtTime(ctx.c.now() + start_in as f64),
        quarters: None,
        run: Arc::new(move |c, now, on| {
            let (at, wait) = landing(c, now, on)?;
            let pace = c.pace();
            let over = over.seconds(c, at, pace.over);
            c.gesture(&param, to, over, curve.unwrap_or(pace.curve), wait)
                .map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_harmony(ctx: &Ctx, a: HarmonyArgs) -> Result<Action, String> {
    if a.stop.unwrap_or(false) {
        return Ok(Action {
            kind: Kind::Stop,
            what: "harmony stop".into(),
            own: StartOn::Now,
            quarters: None,
            run: Arc::new(|c, _, _| {
                c.stop_harmony();
                Ok(())
            }),
        });
    }
    ctx.no_timing(&[
        ("start_on", a.start_on.is_some()),
        ("at_bar", a.at_bar.is_some()),
    ])?;
    let key_name = a
        .key
        .ok_or("harmony needs a key (e.g. \"A\"), or stop: true")?;
    let key =
        Key::new(&key_name, a.mode.as_deref().unwrap_or("aeolian")).map_err(|e| e.to_string())?;
    let steps = a
        .progression
        .filter(|p| !p.is_empty())
        .ok_or("harmony needs a progression with at least one chord")?;
    if steps.len() > MAX_CHORDS {
        return Err(format!(
            "a progression can have at most {MAX_CHORDS} chords"
        ));
    }
    let mut progression = Vec::with_capacity(steps.len());
    for (i, step) in steps.iter().enumerate() {
        let chord = Chord::parse(&step.chord, &key).map_err(|e| format!("chord {}: {e}", i + 1))?;
        let length =
            chord_length(step).map_err(|e| format!("chord {} ({}): {e}", i + 1, step.chord))?;
        progression.push((chord, length));
    }
    let pedal = a
        .pedal
        .as_deref()
        .filter(|p| !p.trim().is_empty())
        .map(parse_note)
        .transpose()
        .map_err(|e| format!("pedal: {e}"))?;
    if let Some(midi) = pedal.and_then(|p| p.midi) {
        let (lo, hi) = moodscore_director::harmony::BASS_RANGE;
        if !(lo..=hi).contains(&(midi as f32)) {
            return Err("pedal: give the note without an octave, or between E1 and E2".into());
        }
    }
    let glide = a.glide.unwrap_or(DEFAULT_GLIDE);
    if !glide.is_finite() || !(0.0..=600.0).contains(&glide) {
        return Err("glide must be between 0 and 600 seconds".into());
    }
    let curve = curve_or(a.curve.as_deref(), Curve::EaseInOut)?;
    let own = if ctx.in_section {
        StartOn::Now
    } else {
        start_on(a.start_on.as_deref(), a.at_bar)?
    };
    let chords: Vec<String> = steps.iter().map(|s| s.chord.clone()).collect();
    let plan = HarmonyPlan {
        key,
        progression,
        looping: a.looping.unwrap_or(true),
        pedal,
        glide,
        curve,
        start_on: StartOn::Now,
    };
    let reclaim = a.reclaim.unwrap_or(false);
    // Check on a copy (the instrument has voices to harmonise, say).
    let (mut d, now) = ctx.c.snapshot();
    d.harmony(now, plan.clone(), reclaim)
        .map_err(|e| e.to_string())?;
    Ok(Action {
        kind: Kind::Harmony,
        what: format!("harmony {key_name} {}", chords.join(" ")),
        own,
        quarters: None,
        run: Arc::new(move |c, _now, on| {
            let mut plan = plan.clone();
            plan.start_on = on;
            c.harmony(plan, reclaim).map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_tempo(ctx: &Ctx, a: TempoArgs) -> Result<Action, String> {
    ctx.no_timing(&[
        ("start_on", a.start_on.is_some()),
        ("at_bar", a.at_bar.is_some()),
    ])?;
    let meter: Option<Meter> = a
        .meter
        .as_deref()
        .filter(|m| !m.trim().is_empty())
        .map(str::parse)
        .transpose()?;
    let curve = curve_or(a.curve.as_deref(), Curve::Linear)?;
    let downbeat = a.downbeat.unwrap_or(false);
    if a.bpm.is_none() && meter.is_none() && !downbeat {
        return Err("tempo needs a bpm, a meter or downbeat: true".into());
    }
    let ramp = match (a.over, a.over_bars) {
        (Some(_), Some(_)) => return Err("give over or over_bars, not both".into()),
        (Some(s), None) => Ramp::Seconds(s),
        (None, Some(bars)) => {
            if !bars.is_finite() || !(0.0..=MAX_BARS).contains(&bars) {
                return Err(format!("over_bars must be between 0 and {MAX_BARS}"));
            }
            // Bars of the new metre, counted when it happens.
            Ramp::Quarters(bars)
        }
        (None, None) => Ramp::Seconds(0.0),
    };
    let own = if ctx.in_section {
        StartOn::Now
    } else {
        start_on(a.start_on.as_deref(), a.at_bar)?
    };
    let bpm = a.bpm;
    let resolve = move |c: &Conductor, at: f64| match ramp {
        Ramp::Quarters(bars) => {
            let m = meter.unwrap_or_else(|| c.tempo().meter_at(at));
            Ramp::Quarters(bars * m.quarters_per_bar())
        }
        r => r,
    };
    // Check on a copy: bpm and glide in range.
    let (mut d, now) = ctx.c.snapshot();
    d.tempo_at(
        now,
        StartOn::Now,
        bpm,
        meter,
        resolve(ctx.c, now),
        curve,
        downbeat,
    )
    .map_err(|e| e.to_string())?;
    let mut what = String::from("tempo");
    if let Some(b) = bpm {
        what.push_str(&format!(" {b}"));
    }
    if let Some(m) = meter {
        what.push_str(&format!(" {m}"));
    }
    Ok(Action {
        kind: Kind::Tempo,
        what,
        own,
        quarters: None,
        run: Arc::new(move |c, now, on| {
            let at = c.start_time(on).map_err(|e| e.to_string())?;
            let ramp = resolve(c, at);
            c.with_director(|d| d.tempo_at(now, on, bpm, meter, ramp, curve, downbeat))
                .map(|_| ())
                .map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_pace(ctx: &Ctx, a: SetPaceArgs) -> Result<Action, String> {
    let curve: Option<Curve> = a
        .curve
        .as_deref()
        .filter(|c| !c.trim().is_empty())
        .map(str::parse)
        .transpose()?;
    let (mut d, _) = ctx.c.snapshot();
    d.set_pace(a.over, curve).map_err(|e| e.to_string())?;
    let over = a.over;
    Ok(Action {
        kind: Kind::Pace,
        what: "set_pace".into(),
        own: StartOn::Now,
        quarters: None,
        run: Arc::new(move |c, _, _| {
            c.set_pace(over, curve)
                .map(|_| ())
                .map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_patch(ctx: &Ctx, a: PatchArgs) -> Result<Action, String> {
    let part = notes::part_index(a.part.as_ref())?;
    let over = a.over.unwrap_or(0.0);
    if !over.is_finite() || !(0.0..=600.0).contains(&over) {
        return Err("over must be between 0 and 600 seconds".into());
    }
    let set = match &a.set {
        Some(m) => notes::parse_set(m)?,
        None => Vec::new(),
    };
    let preset = a
        .preset
        .as_deref()
        .filter(|p| !p.trim().is_empty())
        .map(notes::find_preset)
        .transpose()?;
    let all: Option<Vec<(String, f32)>> = preset.as_ref().map(|p| {
        // Every parameter, so earlier tweaks don't linger.
        moodscore_engine::synth_params::PARAMS
            .iter()
            .zip(&p.patch.0)
            .map(|(d, &v)| (d.path.to_string(), v))
            .collect()
    });
    let (mut d, now) = ctx.c.snapshot();
    if let Some(all) = &all {
        d.set_params(now, &notes::to_changes(part, all), 0.0)
            .map_err(|e| e.to_string())?;
    }
    if !set.is_empty() {
        d.set_params(now, &notes::to_changes(part, &set), over)
            .map_err(|e| e.to_string())?;
    }
    let mut what = format!("patch part {}", part + 1);
    if let Some(p) = &preset {
        what.push_str(&format!(" {}", p.name));
    }
    let loaded = ctx.loaded.clone();
    let preset = preset.map(|p| (p.name.clone(), p.patch.clone()));
    Ok(Action {
        kind: Kind::Patch,
        what,
        own: StartOn::Now,
        quarters: None,
        run: Arc::new(move |c, _, _| {
            if let Some(all) = &all {
                c.set_params(&notes::to_changes(part, all), 0.0)
                    .map_err(|e| e.to_string())?;
                loaded.lock().unwrap_or_else(|e| e.into_inner())[part] = preset.clone();
            }
            if !set.is_empty() {
                c.set_params(&notes::to_changes(part, &set), over)
                    .map_err(|e| e.to_string())?;
            }
            Ok(())
        }),
    })
}

pub fn prepare_notes(ctx: &Ctx, a: PlayNotesArgs) -> Result<Action, String> {
    let part = notes::part_index(a.part.as_ref())?;
    if a.stop.unwrap_or(false) {
        return Ok(Action {
            kind: Kind::Stop,
            what: format!("play_notes stop part {}", part + 1),
            own: StartOn::Now,
            quarters: None,
            run: Arc::new(move |c, _, _| {
                c.stop_notes(part);
                Ok(())
            }),
        });
    }
    ctx.no_timing(&[
        ("start_on", a.start_on.is_some()),
        ("at_bar", a.at_bar.is_some()),
    ])?;
    let repeat = a.repeat.unwrap_or(1);
    if !(1..=MAX_REPEAT).contains(&repeat) {
        return Err(format!("repeat must be between 1 and {MAX_REPEAT}"));
    }
    let velocity = a.velocity.unwrap_or(0.8);
    let gate = a.gate.unwrap_or(0.9);
    let check = |velocity: f32, gate: f32, beats: f32| -> Result<(), String> {
        if !beats.is_finite() || !(0.0625..=64.0).contains(&beats) {
            return Err("beats must be between 1/16 and 64".into());
        }
        if !velocity.is_finite() || !(0.0..=1.0).contains(&velocity) {
            return Err("velocity must be between 0 and 1".into());
        }
        if !gate.is_finite() || !(0.05..=2.0).contains(&gate) {
            return Err("gate must be between 0.05 and 2".into());
        }
        Ok(())
    };
    check(velocity, gate, 1.0)?;
    let steps: Vec<notes::Step> = match (a.phrase.as_deref(), &a.steps) {
        (Some(_), Some(s)) if !s.is_empty() => return Err("give phrase or steps, not both".into()),
        (Some(p), _) if !p.trim().is_empty() => {
            let steps = notes::parse_phrase(p, velocity, gate)?;
            for (i, s) in steps.iter().enumerate() {
                check(s.velocity, s.gate, s.beats).map_err(|e| format!("token {}: {e}", i + 1))?;
            }
            steps
        }
        (_, Some(raw)) if !raw.is_empty() => {
            let mut steps = Vec::with_capacity(raw.len());
            for (i, s) in raw.iter().enumerate() {
                let at = |e: String| format!("step {} ({}): {e}", i + 1, s.notes);
                let beats = s.beats.unwrap_or(1.0);
                let (v, g) = (s.velocity.unwrap_or(velocity), s.gate.unwrap_or(gate));
                check(v, g, beats).map_err(at)?;
                let notes = notes::parse_notes(&s.notes).map_err(at)?;
                if notes.len() > 8 {
                    return Err(at("at most 8 notes at once".into()));
                }
                steps.push(notes::Step {
                    notes,
                    beats,
                    velocity: v,
                    gate: g,
                });
            }
            steps
        }
        _ => {
            return Err(
                "play_notes needs a phrase, e.g. \"A3:1 C4 E4:2\", or steps, or stop: true".into(),
            )
        }
    };
    if steps.len() > MAX_STEPS {
        return Err(format!("a phrase can have at most {MAX_STEPS} steps"));
    }
    let events: usize = steps.iter().map(|s| s.notes.len() * 2).sum::<usize>() * repeat as usize;
    if events > MAX_EVENTS {
        return Err(format!(
            "that is {events} note events; at most {MAX_EVENTS} per call (shorten the phrase or repeat less)"
        ));
    }
    let own = if ctx.in_section {
        StartOn::Now
    } else {
        start_on(a.start_on.as_deref(), a.at_bar)?
    };
    let per_beat = ctx.c.tempo().meter().quarters_per_beat();
    let total = steps.iter().map(|s| s.beats as f64 * per_beat).sum::<f64>() * repeat as f64;
    Ok(Action {
        kind: Kind::Notes,
        what: format!("play_notes part {} ({} steps)", part + 1, steps.len()),
        own,
        quarters: Some(total),
        run: Arc::new(move |c, _now, on| {
            let tempo = c.tempo();
            let start = c.start_time(on).map_err(|e| e.to_string())?;
            let events = notes::schedule(&tempo, start, &steps, repeat);
            c.schedule_notes(part, &events).map_err(|e| e.to_string())
        }),
    })
}

pub fn prepare_sequence(ctx: &Ctx, a: SequenceArgs) -> Result<Action, String> {
    let part = notes::part_index(a.part.as_ref())?;
    ctx.no_timing(&[
        ("start_on", a.start_on.is_some()),
        ("at_bar", a.at_bar.is_some()),
    ])?;
    let own = if ctx.in_section {
        StartOn::Now
    } else {
        start_on_bar(a.start_on.as_deref(), a.at_bar)?
    };
    if a.stop.unwrap_or(false) {
        return Ok(Action {
            kind: Kind::Stop,
            what: format!("sequence stop part {}", part + 1),
            own,
            quarters: None,
            run: Arc::new(move |c, _, on| c.stop_sequence(part, on).map_err(|e| e.to_string())),
        });
    }
    let bars = a.bars;
    if let Some(b) = bars {
        if !b.is_finite() || !(0.25..=MAX_BARS).contains(&b) {
            return Err(format!("bars must be between 0.25 and {MAX_BARS}"));
        }
    }
    let (pattern, plays) = a.spec.pattern()?;
    Ok(Action {
        kind: Kind::Notes,
        what: format!("sequence part {}: {plays}", part + 1),
        own,
        quarters: None,
        run: Arc::new(move |c, _, on| {
            c.sequence(part, pattern.clone(), plays.clone(), on, bars)
                .map(|_| ())
                .map_err(|e| e.to_string())
        }),
    })
}
