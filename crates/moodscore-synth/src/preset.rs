//! Presets: named patches as data, and plain-language descriptions.
//!
//! A preset file lists only what differs from the default patch:
//!
//! ```json
//! { "name": "pluck", "description": "…", "params": { "filter/cutoff": 380, "mod1/source": "velocity" } }
//! ```
//!
//! Choices are written by name. Factory presets live in `presets/` at the
//! root of the repository and are built into the binary.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::params::{find, Kind, Patch, MOD_DESTS, MOD_SLOTS, MOD_SOURCES, P, PARAMS};

/// Presets built into the binary.
pub const FACTORY: &[&str] = &[
    include_str!("../../../presets/pluck.json"),
    include_str!("../../../presets/lead.json"),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Preset {
    pub name: String,
    pub description: String,
    pub patch: Patch,
}

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    params: Map<String, Value>,
}

/// Parse a value for a parameter: a number, or a choice by name.
pub fn parse_value(path: &str, v: &Value) -> Result<f32, String> {
    let id = find(path).ok_or_else(|| format!("unknown parameter '{path}'"))?;
    let def = &PARAMS[id];
    let x = match v {
        Value::Number(n) => n.as_f64().unwrap_or(f64::NAN) as f32,
        Value::Bool(b) => *b as u8 as f32,
        Value::String(s) => match def.kind {
            Kind::Choice(names) => def
                .choice_index(s)
                .ok_or_else(|| format!("{path}: '{s}' is not one of {}", names.join(", ")))?,
            _ => s
                .trim()
                .parse::<f32>()
                .map_err(|_| format!("{path}: '{s}' is not a number"))?,
        },
        _ => return Err(format!("{path}: give a number or a name")),
    };
    if !x.is_finite() || x < def.min || x > def.max {
        return Err(format!(
            "{path} = {x} is outside {} .. {}",
            def.min, def.max
        ));
    }
    Ok(def.sanitize(x))
}

/// A parameter's value as written in a preset: a number, or a choice's name.
pub fn value_json(id: usize, v: f32) -> Value {
    let def = &PARAMS[id];
    if let Some(name) = def.choice_name(v) {
        return Value::String(name.into());
    }
    let r = (v as f64 * 1000.0).round() / 1000.0;
    serde_json::Number::from_f64(r).map_or(Value::Null, Value::Number)
}

impl Preset {
    pub fn parse(json: &str) -> Result<Self, String> {
        let f: File = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut patch = Patch::default();
        for (path, v) in &f.params {
            let x = parse_value(path, v)?;
            patch.0[find(path).unwrap_or(0)] = x;
        }
        Ok(Self {
            name: f.name,
            description: f.description,
            patch,
        })
    }

    pub fn to_json(&self) -> String {
        let f = File {
            name: self.name.clone(),
            description: self.description.clone(),
            params: changed(&self.patch),
        };
        serde_json::to_string_pretty(&f).unwrap_or_default()
    }

    pub fn factory() -> Vec<Preset> {
        FACTORY
            .iter()
            .map(|j| Preset::parse(j).expect("factory preset"))
            .collect()
    }
}

/// The parameters that differ from the default patch, as preset values.
pub fn changed(patch: &Patch) -> Map<String, Value> {
    let mut m = Map::new();
    for (id, def) in PARAMS.iter().enumerate() {
        let v = patch.0[id];
        if (v - def.init).abs() > 1e-6 * def.init.abs().max(1.0) {
            m.insert(def.path.into(), value_json(id, v));
        }
    }
    m
}

/// A short, musical description of a patch.
pub fn describe(patch: &Patch) -> String {
    let g = |p: P| patch.0[p as usize];
    let d = |p: P| p.def().sanitize(g(p));
    let mut parts: Vec<String> = Vec::new();

    let voices = d(P::PartVoices) as usize;
    let analog = g(P::Analog);
    parts.push(if voices == 1 {
        let mut s = "monophonic".to_string();
        if d(P::PartLegato) >= 0.5 {
            s += ", legato";
        }
        if g(P::PartGlide) > 0.0 {
            s += &format!(", glide {}", secs(g(P::PartGlide)));
        }
        s
    } else {
        format!("{voices} voices")
    });

    if analog > 0.0 {
        parts.push(format!("analog drift {analog:.2}"));
    }
    for (n, level, shape, oct, semi, uni, det, sync) in [
        (
            1,
            P::Osc1Level,
            P::Osc1Shape,
            P::Osc1Octave,
            P::Osc1Semi,
            P::Osc1Unison,
            P::Osc1Detune,
            P::Osc1Sync,
        ),
        (
            2,
            P::Osc2Level,
            P::Osc2Shape,
            P::Osc2Octave,
            P::Osc2Semi,
            P::Osc2Unison,
            P::Osc2Detune,
            P::Osc2Sync,
        ),
    ] {
        if g(level) <= 0.0 && !(n == 2 && g(P::FmAmount) > 0.0) {
            continue;
        }
        let mut s = format!("osc{n} {}", shape_name(g(shape)));
        let transpose = 12.0 * d(oct) + d(semi);
        if transpose != 0.0 {
            s += &format!(" {transpose:+} semitones");
        }
        let u = d(uni) as usize;
        if u > 1 {
            s += &format!(", {u} copies {} cents apart", g(det).round());
        }
        if g(sync) > 1.0005 {
            s += &format!(", synced ×{:.2}", g(sync));
        }
        if g(level) <= 0.0 {
            s += " (silent, FM only)";
        } else if (g(level) - 1.0).abs() > 0.01 {
            s += &format!(" at {:.2}", g(level));
        }
        parts.push(s);
    }
    if g(P::FmAmount) > 0.0 {
        parts.push(format!("FM {:.2}", g(P::FmAmount)));
    }
    if g(P::SubLevel) > 0.0 {
        parts.push(format!(
            "sub {} octave(s) down at {:.2}",
            -d(P::SubOctave),
            g(P::SubLevel)
        ));
    }
    if g(P::NoiseLevel) > 0.0 {
        parts.push(format!("noise {:.2}", g(P::NoiseLevel)));
    }
    if g(P::Fold) > 0.0 {
        parts.push(format!("fold {:.2}", g(P::Fold)));
    }
    if g(P::Drive) > 0.0 {
        parts.push(format!("drive {:.2}", g(P::Drive)));
    }

    let ftype = P::FilterType
        .def()
        .choice_name(g(P::FilterType))
        .unwrap_or("low_pass");
    let mut f = if ftype == "ladder" {
        let mut f = format!("ladder (Moog-style) 24 dB at {} Hz", g(P::Cutoff).round());
        if g(P::FilterDrive) > 0.0 {
            f += &format!(", driven {:.2}", g(P::FilterDrive));
        }
        f
    } else {
        format!(
            "{} {} dB at {} Hz",
            ftype.replace('_', "-"),
            d(P::FilterSlope),
            g(P::Cutoff).round()
        )
    };
    if g(P::Resonance) > 0.15 {
        f += &format!(", resonance {:.2}", g(P::Resonance));
    }
    if g(P::FilterEnv) != 0.0 {
        f += &format!(
            ", envelope {:+.1} octaves ({})",
            g(P::FilterEnv),
            env_shape(
                g(P::FenvAttack),
                g(P::FenvDecay),
                g(P::FenvSustain),
                g(P::FenvRelease)
            )
        );
    }
    parts.push(f);
    parts.push(format!(
        "volume {}",
        env_shape(
            g(P::AmpAttack),
            g(P::AmpDecay),
            g(P::AmpSustain),
            g(P::AmpRelease)
        )
    ));

    for slot in 0..MOD_SLOTS {
        let base = P::Mod1Source as usize + 3 * slot;
        let src = PARAMS[base].sanitize(patch.0[base]) as usize;
        let dst = PARAMS[base + 1].sanitize(patch.0[base + 1]) as usize;
        let amount = patch.0[base + 2];
        if src != 0 && dst != 0 && amount != 0.0 {
            parts.push(format!(
                "{} → {} {:+.3}",
                MOD_SOURCES[src], MOD_DESTS[dst], amount
            ));
        }
    }

    if g(P::Distortion) > 0.0 {
        parts.push(format!("distortion {:.2}", g(P::Distortion)));
    }
    if g(P::Ensemble) > 0.0 {
        parts.push(format!("ensemble {:.2}", g(P::Ensemble)));
    }
    if g(P::ChorusMix) > 0.0 {
        parts.push(format!("chorus {:.2}", g(P::ChorusMix)));
    }
    if g(P::DelayMix) > 0.0 {
        let div = P::DelayDivision
            .def()
            .choice_name(g(P::DelayDivision))
            .unwrap_or("?");
        parts.push(format!(
            "echo {div} at {:.2}, feedback {:.2}",
            g(P::DelayMix),
            g(P::DelayFeedback)
        ));
    }
    parts.push(format!(
        "level {:.2}, reverb send {:.2}",
        g(P::PartLevel),
        g(P::ReverbSend)
    ));
    parts.join("; ")
}

fn secs(s: f32) -> String {
    if s < 0.1 {
        format!("{} ms", (s * 1000.0).round())
    } else {
        format!("{s:.2} s")
    }
}

fn env_shape(a: f32, d: f32, s: f32, r: f32) -> String {
    let attack = format!("attack {}", secs(a));
    if s <= 0.01 {
        format!(
            "{attack}, dies away in {} (no sustain), release {}",
            secs(d),
            secs(r)
        )
    } else if s >= 0.99 {
        format!("{attack}, holds, release {}", secs(r))
    } else {
        format!(
            "{attack}, decay {} to {:.2}, release {}",
            secs(d),
            s,
            secs(r)
        )
    }
}

fn shape_name(m: f32) -> String {
    const NAMES: [&str; 4] = ["sine", "triangle", "saw", "square"];
    let k = m.round();
    if (m - k).abs() < 0.05 {
        NAMES[k as usize].to_string()
    } else {
        let lo = m.floor() as usize;
        format!(
            "{:.2} (between {} and {})",
            m,
            NAMES[lo],
            NAMES[(lo + 1).min(3)]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_presets_parse_and_round_trip() {
        let all = Preset::factory();
        assert_eq!(all.len(), 2);
        for p in &all {
            assert!(!p.description.is_empty());
            let back = Preset::parse(&p.to_json()).unwrap();
            assert_eq!(&back, p, "{}", p.name);
        }
        let lead = &all[1];
        assert_eq!(lead.patch.get(P::PartVoices), 1.0);
        let text = describe(&lead.patch);
        assert!(text.contains("monophonic, legato"), "{text}");
        assert!(text.contains("lfo1 → pitch/bend"), "{text}");
    }

    #[test]
    fn bad_values_are_explained() {
        let e = Preset::parse(r#"{"name":"x","params":{"filter/type":"notch"}}"#).unwrap_err();
        assert!(e.contains("low_pass"), "{e}");
        let e = Preset::parse(r#"{"name":"x","params":{"filter/cutoff":5}}"#).unwrap_err();
        assert!(e.contains("outside"), "{e}");
        let e = Preset::parse(r#"{"name":"x","params":{"wobble":1}}"#).unwrap_err();
        assert!(e.contains("unknown"), "{e}");
    }
}
