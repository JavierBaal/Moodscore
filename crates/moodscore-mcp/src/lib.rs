//! Moodscore MCP server.
//!
//! The tools an AI agent uses to play Moodscore, as described in
//! `docs/MCP_TOOLS.md`: `start` and `release` (the transport), `morph`
//! (the mood macros), `gesture`, `set_pace` and `get_state`.
//!
//! Every tool returns the resulting state as JSON, so the agent never has to
//! guess what the instrument is doing.

use moodscore_director::{Conductor, Curve};
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
`gesture` for a specific touch the macros don't cover, such as retuning the \
drone with drone_1/pitch.
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

/// How a value travels from where it is to where it is going.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CurveArg {
    /// Constant speed.
    Linear,
    /// Starts slowly, speeds up.
    EaseIn,
    /// Starts quickly, settles gently.
    EaseOut,
    /// Slow start and finish. The default.
    EaseInOut,
    /// Barely moves, then rushes to the end. For arrivals and hits.
    Exponential,
    /// Moves most at first, then creeps toward the end.
    Logarithmic,
}

impl From<CurveArg> for Curve {
    fn from(c: CurveArg) -> Self {
        match c {
            CurveArg::Linear => Curve::Linear,
            CurveArg::EaseIn => Curve::EaseIn,
            CurveArg::EaseOut => Curve::EaseOut,
            CurveArg::EaseInOut => Curve::EaseInOut,
            CurveArg::Exponential => Curve::Exponential,
            CurveArg::Logarithmic => Curve::Logarithmic,
        }
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
    /// Shape of the movement. Default: the current pace.
    pub curve: Option<CurveArg>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GestureArgs {
    /// Parameter path, e.g. "drone_1/pitch" or "filter/cutoff". `get_state` lists them all with their ranges.
    pub param: String,
    /// Target value, inside the parameter's range.
    pub to: f32,
    /// Seconds to get there. 0 means as fast as is musically safe. Default: the current pace.
    pub over: Option<f32>,
    /// Shape of the movement. Default: the current pace.
    pub curve: Option<CurveArg>,
    /// Seconds to wait before starting. Default 0.
    pub start_in: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetStateArgs {}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetPaceArgs {
    /// Default seconds for every move that doesn't give its own. 0 means as fast as is musically safe. Omit to keep the current value.
    pub over: Option<f32>,
    /// Default shape for every move that doesn't give its own. Omit to keep the current value.
    pub curve: Option<CurveArg>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StartArgs {
    /// Seconds to fade in. Default 3.
    pub over: Option<f32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReleaseArgs {
    /// Seconds to fade out to silence. Default 8. Use 1 or less only for an urgent stop.
    pub over: Option<f32>,
}

const DEFAULT_START: f32 = 3.0;
const DEFAULT_RELEASE: f32 = 8.0;
const MAX_FADE: f32 = 600.0;

fn check_fade(over: f32) -> Result<f32, String> {
    if over.is_finite() && (0.0..=MAX_FADE).contains(&over) {
        Ok(over)
    } else {
        Err(format!("'over' must be between 0 and {MAX_FADE} seconds"))
    }
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

#[derive(Serialize)]
struct Output {
    /// True while sounding or fading in; false when released or fading out.
    playing: bool,
    /// Current output gain, 0 (silent) to 1.
    gain: f32,
    rms_dbfs: f32,
    peak_dbfs: f32,
    sample_rate: u32,
}

#[tool_router]
impl MoodscoreServer {
    pub fn new(conductor: Conductor) -> Self {
        Self {
            conductor,
            tool_router: Self::tool_router(),
        }
    }

    fn report(&self) -> String {
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
            },
            state: self.conductor.state(),
        };
        serde_json::to_string_pretty(&report).unwrap_or_else(|e| format!("{{\"error\":\"{e}\"}}"))
    }

    #[tool(
        description = "Start sounding: fade in from silence. Moodscore starts silent; call this when the person wants to hear it. After a release, brings the sound back exactly as it was. Returns the new state."
    )]
    fn start(&self, Parameters(args): Parameters<StartArgs>) -> Result<String, String> {
        let over = check_fade(args.over.unwrap_or(DEFAULT_START))?;
        self.conductor.engine().fade_to(1.0, over);
        Ok(self.report())
    }

    #[tool(
        description = "Fade everything out to silence. Use when the person says stop, quiet, enough, or for the end of a scene. The instrument keeps its state; `start` brings it back. Returns the new state."
    )]
    fn release(&self, Parameters(args): Parameters<ReleaseArgs>) -> Result<String, String> {
        let over = check_fade(args.over.unwrap_or(DEFAULT_RELEASE))?;
        self.conductor.engine().fade_to(0.0, over);
        Ok(self.report())
    }

    #[tool(
        description = "Move the mood. Glides one or more mood macros to new values over time; each macro moves several sound parameters together. Macros: darkness, instability. Omitted macros keep doing what they were doing. Returns the new state."
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
        let curve = args.curve.map(Curve::from).unwrap_or(pace.curve);
        self.conductor
            .morph(&changes, over, curve)
            .map_err(|e| e.to_string())?;
        Ok(self.report())
    }

    #[tool(
        description = "Move a single sound parameter along a trajectory, for when a mood macro is too coarse (for example retuning the drone with drone_1/pitch). Takes that parameter out of macro control until the next morph of its macro. Returns the new state."
    )]
    fn gesture(&self, Parameters(args): Parameters<GestureArgs>) -> Result<String, String> {
        let pace = self.conductor.pace();
        let over = args.over.unwrap_or(pace.over);
        let curve = args.curve.map(Curve::from).unwrap_or(pace.curve);
        let start_in = args.start_in.unwrap_or(0.0);
        self.conductor
            .gesture(&args.param, args.to, over, curve, start_in)
            .map_err(|e| e.to_string())?;
        Ok(self.report())
    }

    #[tool(
        description = "Set the standing pace: the default duration and curve for every morph and gesture that doesn't give its own. Use it when the person sets a general feel for transitions (\"everything slow today\", \"short and direct\"). Moves already under way keep their timing. Fades from start and release keep their own defaults. Returns the new state."
    )]
    fn set_pace(&self, Parameters(args): Parameters<SetPaceArgs>) -> Result<String, String> {
        self.conductor
            .set_pace(args.over, args.curve.map(Curve::from))
            .map_err(|e| e.to_string())?;
        Ok(self.report())
    }

    #[tool(
        description = "See what Moodscore is doing: whether it is playing, the current pace, every mood macro and parameter with its current value, target, owner and seconds of movement remaining, plus the output level in dBFS."
    )]
    fn get_state(&self, Parameters(_): Parameters<GetStateArgs>) -> String {
        self.report()
    }
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
