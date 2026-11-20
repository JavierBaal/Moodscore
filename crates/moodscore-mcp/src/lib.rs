//! Moodscore MCP server.
//!
//! The tools an AI agent uses to play Moodscore, as described in
//! `docs/MCP_TOOLS.md`: `start` and `release` (the transport), `morph`
//! (the mood macros), `gesture`, `harmony`, `tempo`, `set_pace`, the note
//! parts (`patch`, `play_notes`, `list_presets`, `save_preset`), the ears
//! (`audition`, `listen`, `analyze_file`) and `get_state`.
//!
//! Every tool returns the resulting state as JSON, so the agent never has to
//! guess what the instrument is doing.

mod ears;
mod notes;

use std::sync::{Arc, Mutex};

use moodscore_director::harmony::{parse_note, Chord, Key};
use moodscore_director::{Conductor, Curve, HarmonyPlan, Length, Meter, StartOn};
use moodscore_engine::PARTS;
use moodscore_synth::{Patch, Preset};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{schemars, tool, tool_handler, tool_router, ServerHandler, ServiceExt};
use serde::{Deserialize, Serialize};

/// What the agent reads when it connects.
pub const INSTRUCTIONS: &str = "\
Moodscore is a drone synthesizer for film scoring that you play live. \
The person describes how the scene should feel; you translate that into slow, \
musical movement.

How to play it:
- Moodscore starts silent. Call `start` when the person wants to hear it, and \
`release` when they ask you to stop, go quiet or end. Both fade; nothing about \
the sound is lost, so `start` brings it back as it was.
- Think in trajectories, not values. Every change glides from where the sound \
is now to where you send it, over the time you choose. Drones live on slow \
change: 10 to 60 seconds is normal, 2 to 5 seconds is dramatic, under 1 second \
is a shock.
- Prefer `morph` (mood macros) over `gesture` (single parameters). Use \
`gesture` for a specific touch the macros don't cover. A parameter set with \
`gesture` stays where you put it: later morphs leave it alone (get_state shows \
it as manual, with held_from naming the macro). Pass reclaim: true to `morph` \
to hand such parameters back to the macro.
- There are six drone voices, drone_1 to drone_6. At rest only drone_1 (A1, \
the foundation, with a sub an octave below) and a quiet drone_2 (E2, the \
fifth) sound. drone_3 to drone_6 wait silent (level 0) on A2, C3, E3 and A3, \
an open A minor chord. Bring a voice in or out by gliding drone_N/level with \
`gesture`; retune it with drone_N/pitch (Hz). Building a sound up voice by \
voice is how it grows bigger or more epic; thinning it out is how it empties.
- `harmony` makes the voices play chords: give a key, a mode and a \
progression of Roman-numeral chords with durations, and the engine walks it \
on its own, looping, each voice gliding to the nearest note of the next chord. \
Use it when the person asks for harmony, chords, a progression, a key or a \
change of colour; then bring voices in with drone_N/level so the chord is \
heard. A voice you retune by hand leaves the harmony until reclaim.
- Moodscore keeps a musical clock (tempo and metre, 120 BPM 4/4 until you \
change it). When the person gives a tempo, a metre or a score, set it with \
`tempo` and give chord lengths in bars or beats instead of working out \
seconds: the chords then follow the tempo, accelerandos and ritardandos \
included. A ramp (over or over_bars) speeds up or slows down gradually. \
start_on: \"bar\" makes a new progression land on the next downbeat; \
at_bar lands it on a given bar (get_state shows tempo.bar). Give harmony, \
each play_notes and a release the same at_bar to make layers start and end \
together, instead of counting seconds. A chord with a slash puts that note \
in the bass (I/C#, IVmaj7/A). With short chords, lower glide so each chord \
arrives before the next.
- `release` and `start` accept start_in to delay the fade, e.g. to let an \
unstable outro peak at full volume before it dies away.
- If the person sets a general pace (\"everything slow today\", \"short and \
direct\"), call `set_pace` once instead of repeating `over` and `curve` on every \
call. Give `over` or `curve` explicitly only when a single move should differ \
from the pace. `get_state` reports the current pace.
- You can change your mind at any time: a new instruction starts from wherever \
the sound is, never with a jump.
- Two note parts, note_1 and note_2, play notes over the drones: plucks, \
leads, motifs. Each is a synth you shape: load a preset with `patch` \
(`list_presets` lists them; built in: pluck, lead, moog_bass, swell_horn, \
glide_arp, deep_pad), adjust it with \
`patch` set (every parameter is listed with its range in the patch \
description and in docs), and hear it with `play_notes`, a phrase in beats \
that follows the tempo. Write phrases in the short form (phrase: \
\"B3:0.5 D4 G4:1 | C#4:0.5 E4 A4:1\"): it is compact and has no length \
limit worth worrying about. When a sound is right, `save_preset` keeps it. \
Notes share the drones' room. Describe sounds to the person in musical \
words; move part/expression (with gesture on note_N/part/expression) to \
open or shape a sound while it plays, if the preset routes it.
- You can't hear, so measure: `audition` renders a part's current sound \
playing a phrase, offline (nothing sounds), and reports its brightness, \
harmonics, attack, how it changes over each note, glides, pitch steadiness \
and stereo width, in numbers and words. With a target (list_presets lists \
them) it says what is on target and how far off the rest is. Shape a sound \
by auditioning, adjusting with patch, and auditioning again; then let the \
person judge by ear. `listen` measures the last seconds of the live output \
(the whole mix). `analyze_file` measures a reference WAV the person has put \
in their references folder.
- Call `get_state` to see what is moving, how long it has left, and how loud \
the output is.
- Talk to the person in musical, sensory terms. Don't recite numbers unless \
they ask.";

/// Parse an optional curve name, falling back to `default`.
///
/// Curves are plain strings in the tool schemas (not a JSON-schema enum):
/// clients and models handle a simple string far more reliably.
fn curve_or(arg: Option<&str>, default: Curve) -> Result<Curve, String> {
    match arg {
        None => Ok(default),
        Some(s) if s.trim().is_empty() => Ok(default),
        Some(s) => s.parse(),
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MorphArgs {
    /// Darkness, 0.0 to 1.0. 0 is open and airy; 1 is closed, low and murky.
    pub darkness: Option<f32>,
    /// Instability, 0.0 to 1.0. 0 is steady and pure; 1 is wavering, beating and seasick.
    pub instability: Option<f32>,
    /// Seconds to get there. 0 means as fast as is musically safe. Default: the current pace.
    pub over: Option<f32>,
    /// Shape of the movement: linear, ease_in, ease_out, ease_in_out, exponential or logarithmic. Default: the current pace.
    pub curve: Option<String>,
    /// Also take back parameters these macros drive that were set by hand with `gesture`. Default false: hand-set parameters stay where they were put.
    pub reclaim: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GestureArgs {
    /// Parameter path, e.g. "drone_1/pitch" or "filter/cutoff". `get_state` lists them all with their ranges.
    pub param: String,
    /// Target value, inside the parameter's range.
    pub to: f32,
    /// Seconds to get there. 0 means as fast as is musically safe. Default: the current pace.
    pub over: Option<f32>,
    /// Shape of the movement: linear, ease_in, ease_out, ease_in_out, exponential or logarithmic. Default: the current pace.
    pub curve: Option<String>,
    /// Seconds to wait before starting. Default 0.
    pub start_in: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetStateArgs {}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ChordStep {
    /// Roman numeral in the key: upper case major, lower case minor (i, VI, V, bVII). Add ° dim, + aug, ø half-dim; colour with 7, maj7, sus2, sus4, add9 (e.g. iv7, Vsus4, iadd9).
    pub chord: String,
    /// How long this chord lasts, in seconds. Give exactly one of seconds, beats or bars.
    pub seconds: Option<f32>,
    /// How long this chord lasts, in beats of the current metre (quarter notes in 4/4); follows the tempo.
    pub beats: Option<f32>,
    /// How long this chord lasts, in bars; follows the tempo.
    pub bars: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct HarmonyArgs {
    /// Tonic, e.g. "A", "F#", "Bb". Required unless stop is true.
    pub key: Option<String>,
    /// ionian, dorian, phrygian, lydian, mixolydian, aeolian, locrian (or major, minor). Default aeolian.
    pub mode: Option<String>,
    /// The chords in order, each with its duration. Required unless stop is true.
    pub progression: Option<Vec<ChordStep>>,
    /// Repeat the progression. Default true. If false, the last chord holds.
    #[serde(rename = "loop")]
    pub looping: Option<bool>,
    /// A note held in the bass under every chord, e.g. "A" or "E1". Default: the bass follows each chord's root.
    pub pedal: Option<String>,
    /// Seconds each voice takes to slide to its next note. Default 3.
    pub glide: Option<f32>,
    /// Shape of the slides: linear, ease_in, ease_out, ease_in_out, exponential or logarithmic. Default ease_in_out.
    pub curve: Option<String>,
    /// Also take over voices whose pitch was set by hand with gesture. Default false.
    pub reclaim: Option<bool>,
    /// When the first chord lands: "now", "beat" (next beat) or "bar" (next downbeat). Default now.
    pub start_on: Option<String>,
    /// Land the first chord on the downbeat of this bar instead (see get_state tempo.bar). Use the same bar in play_notes to start layers together.
    pub at_bar: Option<i64>,
    /// Stop walking the harmony: voices stay on their current notes and become free. Other fields are ignored.
    pub stop: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TempoArgs {
    /// Beats per minute, counting the metre's beat (the quarter note in 4/4 and 3/4, the eighth in 6/8). 20 to 300. Omit to keep the current speed.
    pub bpm: Option<f64>,
    /// Time signature, e.g. "4/4", "3/4", "6/8", "5/4". A new metre starts a new bar now. Omit to keep it.
    pub meter: Option<String>,
    /// Seconds to glide to the new bpm (accelerando or ritardando). Default 0: change at once.
    pub over: Option<f64>,
    /// Bars to glide to the new bpm, counted in the new metre. Use instead of over.
    pub over_bars: Option<f64>,
    /// Shape of the glide: linear, ease_in, ease_out, ease_in_out, exponential or logarithmic. Default linear.
    pub curve: Option<String>,
    /// Make this moment the downbeat of bar 1. Default false: the bar count carries on.
    pub downbeat: Option<bool>,
}

const MAX_BARS: f64 = 256.0;

const DEFAULT_GLIDE: f32 = 3.0;
const MAX_CHORDS: usize = 64;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PatchArgs {
    /// Which note part: 1 or 2. Default 1.
    pub part: Option<serde_json::Value>,
    /// Load this preset first (see list_presets), e.g. "pluck", "moog_bass" or "deep_pad".
    pub preset: Option<String>,
    /// Parameters to change, as path → value, e.g. {"filter/cutoff": 1200, "osc1/shape": 2.5, "mod1/source": "lfo1"}. Choices by name. Omit to just describe the patch.
    pub set: Option<serde_json::Map<String, serde_json::Value>>,
    /// Seconds for continuous changes in `set` to glide (counts and choices switch). Default 0.
    pub over: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListPresetsArgs {}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SavePresetArgs {
    /// Which note part's sound to save: 1 or 2. Default 1.
    pub part: Option<serde_json::Value>,
    /// Name for the preset (letters, digits, spaces, _ or -).
    pub name: String,
    /// What it sounds like and what it is for, in a sentence or two.
    pub description: String,
    /// Replace a saved preset of the same name. Default false.
    pub overwrite: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NoteStep {
    /// A note ("A3", "C#4", "Bb2"; C4 is middle C), a chord ("A3 C4 E4"), a MIDI number ("57"), or "rest".
    pub notes: String,
    /// Length in beats of the current metre. Default 1.
    pub beats: Option<f32>,
    /// 0 to 1. Default 0.8.
    pub velocity: Option<f32>,
    /// Fraction of the step the note is held. Default 0.9. Above 1 overlaps the next note (a legato patch then slides).
    pub gate: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PlayNotesArgs {
    /// Which note part: 1 or 2. Default 1.
    pub part: Option<serde_json::Value>,
    /// The phrase in short form: tokens like `A3:0.5` (note:beats), `A3+C4+E4:2` (chord), `r:1` (rest), `D5:1@0.9` (velocity), `F#5:1.5~` (held into the next note: slides on a legato patch). A missing length repeats the previous one; `|` bar lines are ignored. E.g. "B3:0.5 D4 G4:1 | C#4:0.5 E4 A4:1". Use this or steps.
    pub phrase: Option<String>,
    /// The phrase, step by step (the long form of phrase).
    pub steps: Option<Vec<NoteStep>>,
    /// Default velocity for the phrase, 0 to 1. Default 0.8.
    pub velocity: Option<f32>,
    /// Default fraction of each step held. Default 0.9.
    pub gate: Option<f32>,
    /// Play the phrase this many times. Default 1, at most 16.
    pub repeat: Option<u32>,
    /// When it starts: "now" (default), "beat" or "bar".
    pub start_on: Option<String>,
    /// Start on the downbeat of this bar instead (see get_state tempo.bar). Use the same bar in harmony and the other part to start layers together.
    pub at_bar: Option<i64>,
    /// Release every note of the part. Other fields are ignored.
    pub stop: Option<bool>,
}

/// The preset each part last loaded (name and patch), if any.
type Loaded = [Option<(String, Patch)>; PARTS];

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AuditionArgs {
    /// Which note part's current sound: 1 or 2. Default 1.
    pub part: Option<serde_json::Value>,
    /// What to play, in the short phrase form ("A3:2", "A#1:4~ F2:4", "A3+C4+E4:8"). Default: the target's phrase, else "A3:2".
    pub phrase: Option<String>,
    /// Tempo for the phrase. Default: the target's, else the current tempo.
    pub bpm: Option<f64>,
    /// Through the shared room (the reverb and master) as well, with the drones silent. Default false: the part alone.
    pub room: Option<bool>,
    /// Compare with a target (list_presets lists them), e.g. "deep_moog_bass".
    pub target: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ListenArgs {
    /// Seconds of the live output to measure, 1 to 30. Default 5.
    pub seconds: Option<f64>,
    /// [low, high] Hz to look for a pitch in. Default [30, 2000].
    pub pitch_range: Option<(f32, f32)>,
    /// Compare with a target.
    pub target: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AnalyzeFileArgs {
    /// A WAV file in the references folder, by name (e.g. "theme.wav").
    pub file: String,
    /// Start, in seconds. Default 0.
    pub from: Option<f64>,
    /// End, in seconds (at most 60 s after from). Default the end of the file.
    pub to: Option<f64>,
    /// Keep only [low, high] Hz, to single out one instrument in a mix (e.g. [30, 250] for a bass).
    pub band: Option<(f32, f32)>,
    /// [low, high] Hz to look for a pitch in, e.g. [40, 200] for a bass. Default [30, 2000].
    pub pitch_range: Option<(f32, f32)>,
    /// Compare with a target.
    pub target: Option<String>,
}

const MAX_STEPS: usize = 1024;
const MAX_REPEAT: u32 = 64;
const MAX_EVENTS: usize = 8192;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetPaceArgs {
    /// Default seconds for every move that doesn't give its own. 0 means as fast as is musically safe. Omit to keep the current value.
    pub over: Option<f32>,
    /// Default shape for every move that doesn't give its own: linear, ease_in, ease_out, ease_in_out, exponential or logarithmic. Omit to keep the current value.
    pub curve: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StartArgs {
    /// Seconds to fade in. Default 3.
    pub over: Option<f32>,
    /// Seconds to wait before the fade begins. Default 0.
    pub start_in: Option<f32>,
    /// Begin the fade on the downbeat of this bar instead (see get_state tempo.bar).
    pub at_bar: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReleaseArgs {
    /// Seconds to fade out to silence. Default 8. Use 1 or less only for an urgent stop.
    pub over: Option<f32>,
    /// Seconds to wait before the fade begins, e.g. to let a gesture peak at full volume first. Default 0.
    pub start_in: Option<f32>,
    /// Begin the fade on the downbeat of this bar instead (see get_state tempo.bar).
    pub at_bar: Option<i64>,
}

const DEFAULT_START: f32 = 3.0;
const DEFAULT_RELEASE: f32 = 8.0;
const MAX_FADE: f32 = 600.0;

/// `start_in`, or the seconds until bar `at_bar` (not both).
fn wait_for(c: &Conductor, start_in: Option<f32>, at_bar: Option<i64>) -> Result<f32, String> {
    match (start_in, at_bar) {
        (Some(_), Some(_)) => Err("give start_in or at_bar, not both".into()),
        (_, Some(n)) => {
            let t = c.start_time(StartOn::AtBar(n)).map_err(|e| e.to_string())?;
            Ok((t - c.now()).max(0.0) as f32)
        }
        (s, None) => Ok(s.unwrap_or(0.0)),
    }
}

/// Where a plan or phrase starts: `start_on` ("now", "beat", "bar") or `at_bar`.
fn start_on(start_on: Option<&str>, at_bar: Option<i64>) -> Result<StartOn, String> {
    let named = start_on
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty());
    match (named.as_deref(), at_bar) {
        (Some(_), Some(_)) => Err("give start_on or at_bar, not both".into()),
        (_, Some(n)) => Ok(StartOn::AtBar(n)),
        (None | Some("now"), None) => Ok(StartOn::Now),
        (Some("beat"), None) => Ok(StartOn::Beat),
        (Some("bar") | Some("downbeat"), None) => Ok(StartOn::Bar),
        (Some(other), None) => Err(format!(
            "start_on '{other}': use \"now\", \"beat\" or \"bar\" (or at_bar for a bar number)"
        )),
    }
}

fn check_fade(over: f32, start_in: f32) -> Result<(f32, f32), String> {
    if !(over.is_finite() && (0.0..=MAX_FADE).contains(&over)) {
        return Err(format!("'over' must be between 0 and {MAX_FADE} seconds"));
    }
    if !(start_in.is_finite() && (0.0..=MAX_FADE).contains(&start_in)) {
        return Err(format!(
            "'start_in' must be between 0 and {MAX_FADE} seconds"
        ));
    }
    Ok((over, start_in))
}

/// The MCP server: a thin, validating layer over the conductor.
#[derive(Clone)]
pub struct MoodscoreServer {
    conductor: Conductor,
    /// The preset each part last loaded, with its patch (to tell edits).
    loaded: Arc<Mutex<Loaded>>,
    tool_router: ToolRouter<Self>,
}

#[derive(Serialize)]
struct Report {
    seconds_since_start: f64,
    output: Output,
    /// The note parts.
    notes: Vec<notes::PartStatus>,
    #[serde(flatten)]
    state: moodscore_director::State,
}

/// How much of the state a tool returns.
#[derive(Copy, Clone, PartialEq)]
enum Detail {
    /// Everything: `get_state`.
    Full,
    /// After an action: macros without descriptions, and under `moving` only
    /// the hand-set parameters that are moving. Parameters a macro drives move
    /// with it, so the macro entry stands for them.
    Brief,
}

#[derive(Serialize)]
struct Output {
    /// True while sounding or fading in; false when released or fading out.
    playing: bool,
    /// Current output gain, 0 (silent) to 1.
    gain: f32,
    rms_dbfs: f32,
    peak_dbfs: f32,
    sample_rate: u32,
    /// A start or release waiting to begin.
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_fade: Option<moodscore_director::ScheduledFade>,
}

#[tool_router]
impl MoodscoreServer {
    pub fn new(conductor: Conductor) -> Self {
        Self {
            conductor,
            loaded: Arc::new(Mutex::new(Default::default())),
            tool_router: Self::tool_router(),
        }
    }

    fn loaded(&self) -> std::sync::MutexGuard<'_, Loaded> {
        self.loaded.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn part_json(&self, part: usize) -> serde_json::Value {
        let loaded = self.loaded()[part].clone();
        serde_json::to_value(notes::report(&self.conductor, part, &loaded))
            .unwrap_or(serde_json::Value::Null)
    }

    fn report(&self, detail: Detail) -> String {
        let levels = self.conductor.engine().levels();
        let (gain, gain_target) = self.conductor.engine().output_gain();
        let round1 = |x: f32| (x * 10.0).round() / 10.0;
        let report = Report {
            seconds_since_start: (self.conductor.now() * 10.0).round() / 10.0,
            output: Output {
                playing: gain_target > 0.0,
                gain: (gain * 1000.0).round() / 1000.0,
                rms_dbfs: round1(levels.rms_dbfs()),
                peak_dbfs: round1(levels.peak_dbfs()),
                sample_rate: self.conductor.engine().sample_rate(),
                scheduled_fade: self.conductor.scheduled_fade(),
            },
            notes: {
                let loaded = self.loaded().clone();
                (0..PARTS)
                    .map(|p| notes::status(&self.conductor, p, &loaded[p]))
                    .collect()
            },
            state: self.conductor.state(),
        };
        // Through text rather than to_value: f32s keep their short form
        // (0.9, not 0.8999999761581421).
        let mut value: serde_json::Value =
            match serde_json::to_string(&report).and_then(|s| serde_json::from_str(&s)) {
                Ok(v) => v,
                Err(e) => return format!("{{\"error\":\"{e}\"}}"),
            };
        // The note parts' parameters are reported by `patch`, not here.
        if let Some(serde_json::Value::Array(params)) = value.get_mut("params") {
            params.retain(|p| !p["path"].as_str().unwrap_or("").starts_with("note_"));
        }
        if detail == Detail::Brief {
            brief(&mut value);
        }
        serde_json::to_string_pretty(&value).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    #[tool(
        description = "Start sounding: fade in from silence. Moodscore starts silent; call this when the person wants to hear it. After a release, brings the sound back exactly as it was. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn start(&self, Parameters(args): Parameters<StartArgs>) -> Result<String, String> {
        let wait = wait_for(&self.conductor, args.start_in, args.at_bar)?;
        let (over, start_in) = check_fade(args.over.unwrap_or(DEFAULT_START), wait)?;
        self.conductor.fade(1.0, over, start_in);
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Fade everything out to silence. Use when the person says stop, quiet, enough, or for the end of a scene. The instrument keeps its state; `start` brings it back. With start_in the fade waits, e.g. so an unstable outro peaks at full volume before it dies away. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn release(&self, Parameters(args): Parameters<ReleaseArgs>) -> Result<String, String> {
        let wait = wait_for(&self.conductor, args.start_in, args.at_bar)?;
        let (over, start_in) = check_fade(args.over.unwrap_or(DEFAULT_RELEASE), wait)?;
        self.conductor.fade(0.0, over, start_in);
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Move the mood. Glides one or more mood macros to new values over time; each macro moves several sound parameters together. Macros: darkness, instability. Omitted macros keep doing what they were doing. Parameters set by hand with gesture are left alone unless reclaim is true. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn morph(&self, Parameters(args): Parameters<MorphArgs>) -> Result<String, String> {
        let mut changes = Vec::new();
        if let Some(v) = args.darkness {
            changes.push(("darkness", v));
        }
        if let Some(v) = args.instability {
            changes.push(("instability", v));
        }
        let pace = self.conductor.pace();
        let over = args.over.unwrap_or(pace.over);
        let curve = curve_or(args.curve.as_deref(), pace.curve)?;
        self.conductor
            .morph(&changes, over, curve, args.reclaim.unwrap_or(false))
            .map_err(|e| e.to_string())?;
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Move a single sound parameter along a trajectory, for when a mood macro is too coarse (for example bringing in a voice with drone_3/level, or retuning it with drone_3/pitch). Takes that parameter out of macro control: later morphs leave it alone unless they pass reclaim: true. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn gesture(&self, Parameters(args): Parameters<GestureArgs>) -> Result<String, String> {
        let pace = self.conductor.pace();
        let over = args.over.unwrap_or(pace.over);
        let curve = curve_or(args.curve.as_deref(), pace.curve)?;
        let start_in = args.start_in.unwrap_or(0.0);
        self.conductor
            .gesture(&args.param, args.to, over, curve, start_in)
            .map_err(|e| e.to_string())?;
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Set a harmonic plan: a key, a mode and a progression of chords the six drone voices walk through on their own, each voice sliding to the nearest note of the next chord. Mode is colour: aeolian dark, dorian bittersweet, phrygian ominous, lydian luminous and floating, mixolydian open and heroic, ionian plain bright, locrian unstable. Voice levels are untouched, so the chord is heard through whichever voices are sounding: bring voices in with gesture on drone_N/level. A pedal holds one note in the bass while chords change above it, a classic way to build tension. Give each chord's length in seconds, or in bars or beats to follow the tempo set with `tempo` (accelerandos and ritardandos included). A slash puts a note in the bass: I/C#, IVmaj7/A (it wins over a pedal for that chord). A new plan starts from its first chord now (or on the next beat or downbeat with start_on, or on a given bar with at_bar), voice-leading from wherever the voices are, so changing key is just another call. Returns the output, pace, harmony (chord now, next chord, seconds to it) and macros."
    )]
    fn harmony(&self, Parameters(args): Parameters<HarmonyArgs>) -> Result<String, String> {
        if args.stop.unwrap_or(false) {
            self.conductor.stop_harmony();
            return Ok(self.report(Detail::Brief));
        }
        let key_name = args
            .key
            .ok_or("harmony needs a key (e.g. \"A\"), or stop: true")?;
        let key = Key::new(&key_name, args.mode.as_deref().unwrap_or("aeolian"))
            .map_err(|e| e.to_string())?;
        let steps = args
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
            let chord =
                Chord::parse(&step.chord, &key).map_err(|e| format!("chord {}: {e}", i + 1))?;
            let length =
                chord_length(step).map_err(|e| format!("chord {} ({}): {e}", i + 1, step.chord))?;
            progression.push((chord, length));
        }
        let pedal = args
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
        let glide = args.glide.unwrap_or(DEFAULT_GLIDE);
        if !glide.is_finite() || !(0.0..=600.0).contains(&glide) {
            return Err("glide must be between 0 and 600 seconds".into());
        }
        let curve = curve_or(args.curve.as_deref(), Curve::EaseInOut)?;
        let start_on = start_on(args.start_on.as_deref(), args.at_bar)?;
        let plan = HarmonyPlan {
            key,
            progression,
            looping: args.looping.unwrap_or(true),
            pedal,
            glide,
            curve,
            start_on,
        };
        self.conductor
            .harmony(plan, args.reclaim.unwrap_or(false))
            .map_err(|e| e.to_string())?;
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Set the musical clock: tempo (bpm) and metre, at once or gliding over time (accelerando, ritardando). Chords given in bars or beats follow it. The bar count carries on through changes unless downbeat is true; a new metre starts a new bar. Returns the output, pace, tempo (bpm, metre, bar and beat now, any ramp under way), harmony and macros."
    )]
    fn tempo(&self, Parameters(args): Parameters<TempoArgs>) -> Result<String, String> {
        let meter: Option<Meter> = args
            .meter
            .as_deref()
            .filter(|m| !m.trim().is_empty())
            .map(str::parse)
            .transpose()?;
        let curve = curve_or(args.curve.as_deref(), Curve::Linear)?;
        let ramp = match (args.over, args.over_bars) {
            (Some(_), Some(_)) => return Err("give over or over_bars, not both".into()),
            (Some(s), None) => s,
            (None, Some(bars)) => {
                if !bars.is_finite() || !(0.0..=MAX_BARS).contains(&bars) {
                    return Err(format!("over_bars must be between 0 and {MAX_BARS}"));
                }
                match args.bpm {
                    None => 0.0,
                    Some(bpm) => {
                        let m = meter.unwrap_or(self.conductor.tempo().meter());
                        self.conductor.tempo().ramp_seconds_for(
                            self.conductor.now(),
                            bpm,
                            Some(m),
                            bars * m.quarters_per_bar(),
                            curve,
                        )
                    }
                }
            }
            (None, None) => 0.0,
        };
        if args.bpm.is_none() && meter.is_none() && !args.downbeat.unwrap_or(false) {
            return Err("tempo needs a bpm, a meter or downbeat: true".into());
        }
        self.conductor
            .set_tempo(args.bpm, meter, ramp, curve, args.downbeat.unwrap_or(false))
            .map_err(|e| e.to_string())?;
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Shape a note part's sound. Load a preset (preset: \"pluck\"), change parameters (set: {\"filter/cutoff\": 900, \"osc1/unison\": 5, \"mod2/source\": \"lfo2\"}), or with neither just describe it. The synth: two oscillators morphing sine 0, triangle 1, saw 2, square 3 (osc1/shape, pw, octave, semi, fine, unison, detune, spread, sync, random_phase), fm/amount (osc2 on osc1), sub, noise, shape/fold and shape/drive, a filter (filter/type low_pass, band_pass, high_pass, or ladder: the Moog-style four-pole low-pass, thick and saturating, with filter/drive; slope 12 or 24 for the others; cutoff Hz; resonance; keytrack; env_amount in octaves), envelopes amp_env, filter_env, mod_env (attack, decay, sustain, release; amp_env/velocity), part/voices (1 to 4; 1 is monophonic), part/legato, part/glide, part/analog (0 exact, 0.3 alive, 1 an old synth: per-note detune and wander, cutoff and envelope spread), two LFOs (lfo1/shape sine triangle saw square random, rate Hz or sync with division like 1/8 or 1/4d, delay, fade, retrigger), eight modulation slots (modN/source: mod_env, filter_env, amp_env, lfo1, lfo2, velocity, note, random, expression; modN/dest: pitch/bend, voice/gain, voice/pan, osc levels, shapes, pw, fine, detune, sync, fm/amount, fold, drive, filter/cutoff, filter/resonance, filter/drive, lfo rates; modN/amount -1 to 1 as a fraction of the target's range, of its octaves for frequencies), effects (fx/distortion, fx/ensemble for real width (very wide, lows stay mono), chorus_mix, chorus_rate, chorus_depth, delay_mix, delay_division, delay_feedback, delay_width, delay_tone) and part/level, part/pan, part/reverb_send. Returns the part's description and every parameter that differs from the default."
    )]
    fn patch(&self, Parameters(args): Parameters<PatchArgs>) -> Result<String, String> {
        let part = notes::part_index(args.part.as_ref())?;
        let over = args.over.unwrap_or(0.0);
        if !over.is_finite() || !(0.0..=600.0).contains(&over) {
            return Err("over must be between 0 and 600 seconds".into());
        }
        let set = match &args.set {
            Some(m) => notes::parse_set(m)?,
            None => Vec::new(),
        };
        let preset = args
            .preset
            .as_deref()
            .filter(|p| !p.trim().is_empty())
            .map(notes::find_preset)
            .transpose()?;
        if let Some(p) = &preset {
            // Every parameter, so earlier tweaks don't linger.
            let all: Vec<(String, f32)> = moodscore_engine::synth_params::PARAMS
                .iter()
                .zip(&p.patch.0)
                .map(|(d, &v)| (d.path.to_string(), v))
                .collect();
            self.conductor
                .set_params(&notes::to_changes(part, &all), 0.0)
                .map_err(|e| e.to_string())?;
            self.loaded()[part] = Some((p.name.clone(), p.patch.clone()));
        }
        if !set.is_empty() {
            self.conductor
                .set_params(&notes::to_changes(part, &set), over)
                .map_err(|e| e.to_string())?;
        }
        Ok(serde_json::to_string_pretty(&self.part_json(part)).unwrap_or_default())
    }

    #[tool(
        description = "List the note presets (built in and saved with save_preset, each with what it sounds like), the target sounds audition can compare with (each with the phrase and tempo it was measured at, and its criteria), and the folder analyze_file reads reference WAVs from."
    )]
    fn list_presets(&self, Parameters(_): Parameters<ListPresetsArgs>) -> String {
        let list: Vec<serde_json::Value> = notes::all_presets()
            .into_iter()
            .map(|(p, factory)| {
                serde_json::json!({
                    "name": p.name,
                    "description": p.description,
                    "built_in": factory,
                })
            })
            .collect();
        let targets: Vec<serde_json::Value> = moodscore_ear::Target::factory()
            .into_iter()
            .map(|t| {
                serde_json::json!({
                    "name": t.name,
                    "description": t.description,
                    "phrase": t.phrase,
                    "bpm": t.bpm,
                    "criteria": t.criteria,
                })
            })
            .collect();
        serde_json::to_string_pretty(&serde_json::json!({
            "presets": list,
            "targets": targets,
            "references_folder": ears::references_dir().map(|d| d.display().to_string()),
        }))
        .unwrap_or_default()
    }

    #[tool(
        description = "Hear a note part's current sound without playing it: renders a phrase offline (nothing sounds) and measures it. Reports level; brightness (spectral centroid), tilt and noisiness; each note's attack, change in level, brightness at its start, middle and end, harmonics 2 to 8 against the fundamental, tuning, wobble and glide time; and stereo correlation and balance in the lows, mids and highs; plus a summary in words. With target, compares with a target sound (list_presets lists them, with the phrase and tempo they were measured with) and says what is on target and how far off the rest is. Use it in a loop with patch: audition, adjust, audition again. room: true adds the shared reverb."
    )]
    fn audition(&self, Parameters(args): Parameters<AuditionArgs>) -> Result<String, String> {
        let part = notes::part_index(args.part.as_ref())?;
        let target = args
            .target
            .as_deref()
            .filter(|t| !t.trim().is_empty())
            .map(moodscore_ear::Target::find)
            .transpose()?;
        let (report, phrase, bpm) = ears::audition(
            &self.conductor,
            ears::AuditionRequest {
                part,
                phrase: args.phrase.filter(|p| !p.trim().is_empty()),
                bpm: args.bpm,
                room: args.room.unwrap_or(false),
                target,
            },
        )?;
        let mut value = moodscore_ear::to_json(&report);
        if let Some(obj) = value.as_object_mut() {
            obj.insert(
                "played".into(),
                serde_json::json!({
                    "part": moodscore_engine::part_prefix(part),
                    "phrase": phrase,
                    "bpm": (bpm * 10.0).round() / 10.0,
                }),
            );
        }
        Ok(serde_json::to_string_pretty(&value).unwrap_or_default())
    }

    #[tool(
        description = "Measure the last seconds of the live output: the whole mix as it sounds now. Reports level, brightness, tilt, the pitch heard (held notes, glides, wobble), stereo width and balance per band, and how brightness and level moved over the stretch, plus a summary in words. Use it to check the mix (is it too dark, too narrow, leaning to one side, is a part buried) after changing something."
    )]
    fn listen(&self, Parameters(args): Parameters<ListenArgs>) -> Result<String, String> {
        let seconds = args.seconds.unwrap_or(5.0);
        if !(1.0..=moodscore_director::HEARD_SECONDS as f64).contains(&seconds) {
            return Err(format!(
                "seconds must be between 1 and {}",
                moodscore_director::HEARD_SECONDS
            ));
        }
        let target = args
            .target
            .as_deref()
            .map(moodscore_ear::Target::find)
            .transpose()?;
        let report = ears::listen(&self.conductor, seconds, args.pitch_range, target.as_ref())?;
        Ok(serde_json::to_string_pretty(&moodscore_ear::to_json(&report)).unwrap_or_default())
    }

    #[tool(
        description = "Measure a reference recording: a WAV the person put in the references folder (list_presets shows where). Measures the stretch from..to (at most 60 s) as one event: level, brightness and how it moves, tilt, attack and change in level, stereo per band, the pitch heard (held notes, glides, the longest note's harmonics). To single out one instrument in a mix, pick a stretch where it plays alone or leads, and use band (e.g. [30, 250] for a bass) and pitch_range. Use the numbers as a target to shape a sound towards with audition. In a full mix, harmonic levels and glides are approximate."
    )]
    fn analyze_file(
        &self,
        Parameters(args): Parameters<AnalyzeFileArgs>,
    ) -> Result<String, String> {
        let target = args
            .target
            .as_deref()
            .map(moodscore_ear::Target::find)
            .transpose()?;
        let report = ears::analyze_file(ears::FileRequest {
            file: args.file,
            from: args.from,
            to: args.to,
            band: args.band,
            pitch_range: args.pitch_range,
            target,
        })?;
        Ok(serde_json::to_string_pretty(&moodscore_ear::to_json(&report)).unwrap_or_default())
    }

    #[tool(
        description = "Save a note part's current sound as a named preset, with a description of what it sounds like and what it is for, so it can be loaded again with patch in any session."
    )]
    fn save_preset(&self, Parameters(args): Parameters<SavePresetArgs>) -> Result<String, String> {
        let part = notes::part_index(args.part.as_ref())?;
        let name = notes::check_name(&args.name)?;
        let existing = notes::all_presets();
        if let Some((_, factory)) = existing.iter().find(|(p, _)| p.name == name) {
            if *factory {
                return Err(format!(
                    "'{name}' is a built-in preset; choose another name"
                ));
            }
            if !args.overwrite.unwrap_or(false) {
                return Err(format!(
                    "a preset called '{name}' exists; pass overwrite: true to replace it"
                ));
            }
        }
        let dir =
            notes::user_dir().ok_or("no folder to save presets in (set MOODSCORE_PRESETS)")?;
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
        let patch = notes::current_patch(&self.conductor, part);
        let preset = Preset {
            name: name.clone(),
            description: args.description.trim().to_string(),
            patch: patch.clone(),
        };
        let path = dir.join(format!("{name}.json"));
        std::fs::write(&path, preset.to_json())
            .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        self.loaded()[part] = Some((name.clone(), patch));
        Ok(serde_json::to_string_pretty(&serde_json::json!({
            "saved": name,
            "file": path.display().to_string(),
        }))
        .unwrap_or_default())
    }

    #[tool(
        description = "Play a phrase on a note part: notes or chords with lengths in beats, following the tempo (set with tempo), each starting on its exact sample. Prefer the short form, phrase: \"B3:0.5 D4 G4:1 | A3+C#4+E4:2@0.6 r:1 F#5:1.5~\" (note:beats, chords with +, r for rest, @velocity, ~ to hold into the next note; a missing length repeats the last). Up to 1024 steps, repeat up to 64. Start now, on the next beat or bar (start_on), or on a given bar (at_bar: use the same bar as harmony and the other part so layers start together). A new phrase adds to what is waiting; stop: true cancels what is waiting and releases the part. Returns the output and the note parts (notes waiting, when the phrase ends)."
    )]
    fn play_notes(&self, Parameters(args): Parameters<PlayNotesArgs>) -> Result<String, String> {
        let part = notes::part_index(args.part.as_ref())?;
        if args.stop.unwrap_or(false) {
            self.conductor.stop_notes(part);
            return Ok(self.report(Detail::Brief));
        }
        let repeat = args.repeat.unwrap_or(1);
        if !(1..=MAX_REPEAT).contains(&repeat) {
            return Err(format!("repeat must be between 1 and {MAX_REPEAT}"));
        }
        let velocity = args.velocity.unwrap_or(0.8);
        let gate = args.gate.unwrap_or(0.9);
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
        let steps: Vec<notes::Step> = match (args.phrase.as_deref(), &args.steps) {
            (Some(_), Some(s)) if !s.is_empty() => {
                return Err("give phrase or steps, not both".into())
            }
            (Some(p), _) if !p.trim().is_empty() => {
                let steps = notes::parse_phrase(p, velocity, gate)?;
                for (i, s) in steps.iter().enumerate() {
                    check(s.velocity, s.gate, s.beats)
                        .map_err(|e| format!("token {}: {e}", i + 1))?;
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
                    "play_notes needs a phrase, e.g. \"A3:1 C4 E4:2\", or steps, or stop: true"
                        .into(),
                )
            }
        };
        if steps.len() > MAX_STEPS {
            return Err(format!("a phrase can have at most {MAX_STEPS} steps"));
        }
        let tempo = self.conductor.tempo();
        let now = self.conductor.now();
        let start = self
            .conductor
            .start_time(start_on(args.start_on.as_deref(), args.at_bar)?)
            .map_err(|e| e.to_string())?;
        let events = notes::schedule(&tempo, start, &steps, repeat);
        if events.len() > MAX_EVENTS {
            return Err(format!(
                "that is {} note events; at most {MAX_EVENTS} per call (shorten the phrase or repeat less)",
                events.len()
            ));
        }
        self.conductor
            .schedule_notes(part, &events)
            .map_err(|e| e.to_string())?;
        let mut value: serde_json::Value =
            serde_json::from_str(&self.report(Detail::Brief)).unwrap_or_default();
        if let Some(obj) = value.as_object_mut() {
            let total: f64 = steps
                .iter()
                .map(|s| s.beats as f64 * tempo.meter().quarters_per_beat())
                .sum::<f64>()
                * repeat as f64;
            let end = tempo.time_at(tempo.beat_at(start) + total);
            obj.insert(
                "phrase".into(),
                serde_json::json!({
                    "part": moodscore_engine::part_prefix(part),
                    "starts_in": ((start - now).max(0.0) * 100.0).round() / 100.0,
                    "seconds": ((end - start) * 100.0).round() / 100.0,
                }),
            );
        }
        Ok(serde_json::to_string_pretty(&value).unwrap_or_default())
    }

    #[tool(
        description = "Set the standing pace: the default duration and curve for every morph and gesture that doesn't give its own. Use it when the person sets a general feel for transitions (\"everything slow today\", \"short and direct\"). Moves already under way keep their timing. Fades from start and release keep their own defaults. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn set_pace(&self, Parameters(args): Parameters<SetPaceArgs>) -> Result<String, String> {
        self.conductor
            .set_pace(
                args.over,
                args.curve
                    .as_deref()
                    .filter(|c| !c.trim().is_empty())
                    .map(str::parse)
                    .transpose()?,
            )
            .map_err(|e| e.to_string())?;
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "See everything Moodscore is doing: whether it is playing (and any start or release waiting to begin), the current pace, every mood macro and parameter with its current value, target, owner and seconds of movement remaining, plus the output level in dBFS."
    )]
    fn get_state(&self, Parameters(_): Parameters<GetStateArgs>) -> String {
        self.report(Detail::Full)
    }
}

/// A chord's length: exactly one of seconds, beats or bars.
fn chord_length(step: &ChordStep) -> Result<Length, String> {
    let check = |v: f32, lo: f32, hi: f32, unit: &str| {
        if v.is_finite() && (lo..=hi).contains(&v) {
            Ok(v)
        } else {
            Err(format!("{unit} must be between {lo} and {hi}"))
        }
    };
    match (step.seconds, step.beats, step.bars) {
        (Some(s), None, None) => Ok(Length::Seconds(check(s, 0.5, 600.0, "seconds")?)),
        (None, Some(b), None) => Ok(Length::Beats(check(b, 0.5, 1024.0, "beats")?)),
        (None, None, Some(b)) => Ok(Length::Bars(check(b, 0.25, MAX_BARS as f32, "bars")?)),
        (None, None, None) => Err("give its length in seconds, beats or bars".into()),
        _ => Err("give its length in only one of seconds, beats or bars".into()),
    }
}

/// Trim a full report down to what an agent needs after an action.
fn brief(report: &mut serde_json::Value) {
    let Some(obj) = report.as_object_mut() else {
        return;
    };
    if let Some(serde_json::Value::Array(macros)) = obj.get_mut("macros") {
        for m in macros {
            if let Some(m) = m.as_object_mut() {
                m.remove("description");
            }
        }
    }
    let moving: Vec<serde_json::Value> = match obj.remove("params") {
        Some(serde_json::Value::Array(params)) => params
            .into_iter()
            .filter(|p| {
                p["owner"] == "manual" && p["seconds_remaining"].as_f64().unwrap_or(0.0) > 0.0
            })
            .map(|mut p| {
                if let Some(p) = p.as_object_mut() {
                    for k in ["min", "max", "unit"] {
                        p.remove(k);
                    }
                }
                p
            })
            .collect(),
        _ => Vec::new(),
    };
    obj.insert("moving".into(), serde_json::Value::Array(moving));
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for MoodscoreServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("moodscore", env!("CARGO_PKG_VERSION")))
            .with_instructions(INSTRUCTIONS)
    }
}

/// Serve MCP over stdin/stdout until the client disconnects.
///
/// stdout carries the protocol: nothing else may print to it.
pub async fn serve_stdio(conductor: Conductor) -> Result<(), Box<dyn std::error::Error>> {
    let service = MoodscoreServer::new(conductor)
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;
    Ok(())
}
