//! Moodscore MCP server.
//!
//! The tools an AI agent uses to play Moodscore, as described in
//! `docs/MCP_TOOLS.md`: `start` and `release` (the transport), `morph`
//! (the mood macros), `gesture`, `harmony`, `tempo`, `set_pace` and
//! `get_state`.
//!
//! Every tool returns the resulting state as JSON, so the agent never has to
//! guess what the instrument is doing.

use moodscore_director::harmony::{parse_note, Chord, Key};
use moodscore_director::{Conductor, Curve, HarmonyPlan, Length, Meter, StartOn};
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
start_on: \"bar\" makes a new progression land on the next downbeat. With \
short chords, lower glide so each chord arrives before the next.
- `release` and `start` accept start_in to delay the fade, e.g. to let an \
unstable outro peak at full volume before it dies away.
- If the person sets a general pace (\"everything slow today\", \"short and \
direct\"), call `set_pace` once instead of repeating `over` and `curve` on every \
call. Give `over` or `curve` explicitly only when a single move should differ \
from the pace. `get_state` reports the current pace.
- You can change your mind at any time: a new instruction starts from wherever \
the sound is, never with a jump.
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
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReleaseArgs {
    /// Seconds to fade out to silence. Default 8. Use 1 or less only for an urgent stop.
    pub over: Option<f32>,
    /// Seconds to wait before the fade begins, e.g. to let a gesture peak at full volume first. Default 0.
    pub start_in: Option<f32>,
}

const DEFAULT_START: f32 = 3.0;
const DEFAULT_RELEASE: f32 = 8.0;
const MAX_FADE: f32 = 600.0;

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
    tool_router: ToolRouter<Self>,
}

#[derive(Serialize)]
struct Report {
    seconds_since_start: f64,
    output: Output,
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
            tool_router: Self::tool_router(),
        }
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
            state: self.conductor.state(),
        };
        // Through text rather than to_value: f32s keep their short form
        // (0.9, not 0.8999999761581421).
        let mut value: serde_json::Value =
            match serde_json::to_string(&report).and_then(|s| serde_json::from_str(&s)) {
                Ok(v) => v,
                Err(e) => return format!("{{\"error\":\"{e}\"}}"),
            };
        if detail == Detail::Brief {
            brief(&mut value);
        }
        serde_json::to_string_pretty(&value).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    #[tool(
        description = "Start sounding: fade in from silence. Moodscore starts silent; call this when the person wants to hear it. After a release, brings the sound back exactly as it was. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn start(&self, Parameters(args): Parameters<StartArgs>) -> Result<String, String> {
        let (over, start_in) = check_fade(
            args.over.unwrap_or(DEFAULT_START),
            args.start_in.unwrap_or(0.0),
        )?;
        self.conductor.fade(1.0, over, start_in);
        Ok(self.report(Detail::Brief))
    }

    #[tool(
        description = "Fade everything out to silence. Use when the person says stop, quiet, enough, or for the end of a scene. The instrument keeps its state; `start` brings it back. With start_in the fade waits, e.g. so an unstable outro peaks at full volume before it dies away. Returns the output, pace, macros (each standing for the parameters it drives) and any hand-set parameters still moving."
    )]
    fn release(&self, Parameters(args): Parameters<ReleaseArgs>) -> Result<String, String> {
        let (over, start_in) = check_fade(
            args.over.unwrap_or(DEFAULT_RELEASE),
            args.start_in.unwrap_or(0.0),
        )?;
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
        description = "Set a harmonic plan: a key, a mode and a progression of chords the six drone voices walk through on their own, each voice sliding to the nearest note of the next chord. Mode is colour: aeolian dark, dorian bittersweet, phrygian ominous, lydian luminous and floating, mixolydian open and heroic, ionian plain bright, locrian unstable. Voice levels are untouched, so the chord is heard through whichever voices are sounding: bring voices in with gesture on drone_N/level. A pedal holds one note in the bass while chords change above it, a classic way to build tension. Give each chord's length in seconds, or in bars or beats to follow the tempo set with `tempo` (accelerandos and ritardandos included). A new plan starts from its first chord now (or on the next beat or downbeat with start_on), voice-leading from wherever the voices are, so changing key is just another call. Returns the output, pace, harmony (chord now, next chord, seconds to it) and macros."
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
        let start_on = match args
            .start_on
            .as_deref()
            .map(|s| s.trim().to_ascii_lowercase())
            .as_deref()
        {
            None | Some("") | Some("now") => StartOn::Now,
            Some("beat") => StartOn::Beat,
            Some("bar") | Some("downbeat") => StartOn::Bar,
            Some(other) => {
                return Err(format!(
                    "start_on '{other}': use \"now\", \"beat\" or \"bar\""
                ))
            }
        };
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
