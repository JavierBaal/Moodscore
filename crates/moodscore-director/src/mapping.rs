//! Mood macro mappings, loaded from data (`macros/*.json`).

use std::collections::BTreeMap;
use std::fmt;

use moodscore_engine::ParamInfo;
use serde::Deserialize;

/// The mappings shipped with Moodscore.
pub const DEFAULT_MAPPINGS: &str = include_str!("../../../macros/default.json");

/// How a macro's 0..1 value is spread over a parameter's from..to range.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapCurve {
    #[default]
    Linear,
    /// Geometric interpolation: equal steps in ratio. Use for frequencies.
    Exp,
}

#[derive(Debug, Deserialize)]
struct File {
    macros: BTreeMap<String, MacroFile>,
}

#[derive(Debug, Deserialize)]
struct MacroFile {
    #[serde(default)]
    description: String,
    #[serde(default)]
    default: f32,
    targets: Vec<TargetFile>,
}

#[derive(Debug, Deserialize)]
struct TargetFile {
    param: String,
    from: f32,
    to: f32,
    #[serde(default)]
    curve: MapCurve,
}

/// One parameter driven by a macro.
#[derive(Clone, Debug)]
pub struct Target {
    pub param: usize,
    pub from: f32,
    pub to: f32,
    pub curve: MapCurve,
}

impl Target {
    /// Parameter value for macro value `m` in `0..=1`.
    pub fn value(&self, m: f32) -> f32 {
        let m = m.clamp(0.0, 1.0);
        match self.curve {
            MapCurve::Linear => self.from + (self.to - self.from) * m,
            MapCurve::Exp => self.from * (self.to / self.from).powf(m),
        }
    }
}

/// A mood macro: a name, a default, and the parameters it drives.
#[derive(Clone, Debug)]
pub struct MacroDef {
    pub name: String,
    pub description: String,
    pub default: f32,
    pub targets: Vec<Target>,
}

#[derive(Debug, PartialEq)]
pub struct MappingError(pub String);

impl fmt::Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid macro mapping: {}", self.0)
    }
}

impl std::error::Error for MappingError {}

/// Parse and validate mappings against the DSP's parameters.
///
/// Rules: every parameter must exist; from/to must lie inside its range;
/// `exp` needs both ends positive; a parameter can be driven by one macro only.
pub fn load(json: &str, params: &[ParamInfo]) -> Result<Vec<MacroDef>, MappingError> {
    let file: File = serde_json::from_str(json).map_err(|e| MappingError(e.to_string()))?;
    let mut owner: BTreeMap<usize, String> = BTreeMap::new();
    let mut out = Vec::new();

    for (name, m) in file.macros {
        if !(0.0..=1.0).contains(&m.default) {
            return Err(MappingError(format!(
                "macro '{name}': default must be in 0..1"
            )));
        }
        let mut targets = Vec::new();
        for t in m.targets {
            let id = params
                .iter()
                .position(|p| p.path == t.param)
                .ok_or_else(|| {
                    MappingError(format!("macro '{name}': unknown parameter '{}'", t.param))
                })?;
            let p = &params[id];
            for v in [t.from, t.to] {
                if !(p.min..=p.max).contains(&v) {
                    return Err(MappingError(format!(
                        "macro '{name}': {v} is outside the range of '{}' ({} .. {})",
                        p.path, p.min, p.max
                    )));
                }
            }
            if t.curve == MapCurve::Exp && (t.from <= 0.0 || t.to <= 0.0) {
                return Err(MappingError(format!(
                    "macro '{name}': 'exp' on '{}' needs positive from/to",
                    p.path
                )));
            }
            if let Some(other) = owner.insert(id, name.clone()) {
                return Err(MappingError(format!(
                    "'{}' is driven by both '{other}' and '{name}'",
                    p.path
                )));
            }
            targets.push(Target {
                param: id,
                from: t.from,
                to: t.to,
                curve: t.curve,
            });
        }
        out.push(MacroDef {
            name,
            description: m.description,
            default: m.default,
            targets,
        });
    }
    Ok(out)
}
