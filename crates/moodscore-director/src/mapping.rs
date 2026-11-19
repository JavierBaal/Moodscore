//! Mood macro mappings, loaded from data (`macros/*.json`).
//!
//! Each macro drives a list of targets. A target names a parameter (or many,
//! with `*`: `drone_*/drift` is the drift of every drone voice) and how the
//! macro's `0..1` value spreads over it:
//!
//! - `"mode": "set"` (the default): the macro sets the parameter, from `from`
//!   at 0 to `to` at 1. At most one macro may `set` a given parameter.
//! - `"mode": "add"`: the macro adds an offset, from `from` at 0 to `to` at 1,
//!   on top of whatever sets the parameter (or its default, if nothing does).
//!   Any number of macros may `add` to the same parameter. The sum is clamped
//!   to the parameter's range.

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

/// Whether a macro sets a parameter or adds to it.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    #[default]
    Set,
    Add,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(rename = "$comment", default)]
    _comment: String,
    macros: BTreeMap<String, MacroFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MacroFile {
    #[serde(default)]
    description: String,
    #[serde(default)]
    default: f32,
    targets: Vec<TargetFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetFile {
    param: String,
    from: f32,
    to: f32,
    #[serde(default)]
    curve: MapCurve,
    #[serde(default)]
    mode: Mode,
}

/// One parameter driven by a macro.
#[derive(Clone, Debug)]
pub struct Target {
    pub param: usize,
    pub from: f32,
    pub to: f32,
    pub curve: MapCurve,
    pub mode: Mode,
}

impl Target {
    /// The value (for `set`) or offset (for `add`) at macro value `m` in `0..=1`.
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

/// Does `path` match `pattern`? `*` matches any run of characters except `/`.
fn matches(pattern: &str, path: &str) -> bool {
    fn go(p: &[u8], s: &[u8]) -> bool {
        match p.split_first() {
            None => s.is_empty(),
            Some((b'*', rest)) => {
                let mut i = 0;
                loop {
                    if go(rest, &s[i..]) {
                        return true;
                    }
                    if i == s.len() || s[i] == b'/' {
                        return false;
                    }
                    i += 1;
                }
            }
            Some((c, rest)) => s.first() == Some(c) && go(rest, &s[1..]),
        }
    }
    go(pattern.as_bytes(), path.as_bytes())
}

/// Parse and validate mappings against the DSP's parameters.
///
/// Rules: every target must match at least one parameter; `set` values must
/// lie inside the parameter's range, `add` offsets inside its span; `exp`
/// needs both ends positive and only works with `set`; a parameter can be
/// `set` by one macro only, and a macro can drive a parameter only once.
pub fn load(json: &str, params: &[ParamInfo]) -> Result<Vec<MacroDef>, MappingError> {
    let file: File = serde_json::from_str(json).map_err(|e| MappingError(e.to_string()))?;
    let mut setter: BTreeMap<usize, String> = BTreeMap::new();
    let mut out = Vec::new();

    for (name, m) in file.macros {
        if !(0.0..=1.0).contains(&m.default) {
            return Err(MappingError(format!(
                "macro '{name}': default must be in 0..1"
            )));
        }
        let mut targets: Vec<Target> = Vec::new();
        for t in m.targets {
            let ids: Vec<usize> = params
                .iter()
                .enumerate()
                .filter(|(_, p)| matches(&t.param, &p.path))
                .map(|(i, _)| i)
                .collect();
            if ids.is_empty() {
                return Err(MappingError(format!(
                    "macro '{name}': unknown parameter '{}'",
                    t.param
                )));
            }
            if t.curve == MapCurve::Exp && t.mode == Mode::Add {
                return Err(MappingError(format!(
                    "macro '{name}': 'exp' can't be used with 'add' on '{}'",
                    t.param
                )));
            }
            for id in ids {
                let p = &params[id];
                for v in [t.from, t.to] {
                    let ok = match t.mode {
                        Mode::Set => (p.min..=p.max).contains(&v),
                        Mode::Add => v.abs() <= p.max - p.min,
                    };
                    if !ok {
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
                if targets.iter().any(|x| x.param == id) {
                    return Err(MappingError(format!(
                        "macro '{name}' drives '{}' twice",
                        p.path
                    )));
                }
                if t.mode == Mode::Set {
                    if let Some(other) = setter.insert(id, name.clone()) {
                        return Err(MappingError(format!(
                            "'{}' is set by both '{other}' and '{name}' (use \"mode\": \"add\" for one of them)",
                            p.path
                        )));
                    }
                }
                targets.push(Target {
                    param: id,
                    from: t.from,
                    to: t.to,
                    curve: t.curve,
                    mode: t.mode,
                });
            }
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

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn wildcard_matches_within_one_segment() {
        assert!(matches("drone_*/drift", "drone_1/drift"));
        assert!(matches("drone_*/drift", "drone_6/drift"));
        assert!(!matches("drone_*/drift", "drone_1/detune"));
        assert!(!matches("*", "drone_1/drift"));
        assert!(matches("*/*", "drone_1/drift"));
        assert!(matches("filter/cutoff", "filter/cutoff"));
        assert!(!matches("filter/cut", "filter/cutoff"));
    }
}
