//! Targets: what a sound should measure, with tolerances.
//!
//! ```json
//! { "name": "deep_moog_bass", "description": "…",
//!   "criteria": { "harmonic_2_db": [-16, 4], "mid_correlation": [0.95, 0.05] } }
//! ```
//!
//! Each criterion is a metric from [`crate::Report::metrics`], the value it
//! should have, and how far off is still fine.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Report;

/// Targets built into the binary (numbers only, in `presets/targets/`).
pub const FACTORY: &[&str] = &[
    include_str!("../../../presets/targets/deep_moog_bass.json"),
    include_str!("../../../presets/targets/swell_horn.json"),
    include_str!("../../../presets/targets/glide_arp.json"),
    include_str!("../../../presets/targets/deep_pad.json"),
];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Target {
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// The tempo the reference was measured at, for `phrase`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bpm: Option<f32>,
    /// What to play to compare with it (short phrase form, for `audition`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    pub criteria: BTreeMap<String, (f32, f32)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Comparison {
    pub target: String,
    pub passed: usize,
    pub of: usize,
    pub results: Vec<Criterion>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Criterion {
    pub metric: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f32>,
    pub target: f32,
    pub tolerance: f32,
    pub ok: bool,
    /// "on target", "3.5 above", "not measured".
    pub verdict: String,
}

impl Target {
    pub fn parse(json: &str) -> Result<Self, String> {
        let t: Target = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if t.criteria.values().any(|(_, tol)| *tol < 0.0) {
            return Err(format!(
                "target {}: tolerances must not be negative",
                t.name
            ));
        }
        Ok(t)
    }

    pub fn factory() -> Vec<Target> {
        FACTORY
            .iter()
            .map(|j| Target::parse(j).expect("factory target"))
            .collect()
    }

    pub fn find(name: &str) -> Result<Target, String> {
        let all = Self::factory();
        all.iter()
            .find(|t| t.name.eq_ignore_ascii_case(name.trim()))
            .cloned()
            .ok_or_else(|| {
                let names: Vec<&str> = all.iter().map(|t| t.name.as_str()).collect();
                format!("no target called '{name}'; there are: {}", names.join(", "))
            })
    }

    pub fn compare(&self, report: &Report) -> Comparison {
        let results: Vec<Criterion> = self
            .criteria
            .iter()
            .map(|(metric, &(target, tolerance))| {
                let value = report.metrics.get(metric).copied();
                let (ok, verdict) = match value {
                    None => (false, "not measured".to_string()),
                    Some(v) => {
                        let d = v - target;
                        if d.abs() <= tolerance {
                            (true, "on target".to_string())
                        } else {
                            let excess = d.abs() - tolerance;
                            (
                                false,
                                format!(
                                    "{} {} the range",
                                    trim(excess),
                                    if d > 0.0 { "above" } else { "below" }
                                ),
                            )
                        }
                    }
                };
                Criterion {
                    metric: metric.clone(),
                    value,
                    target,
                    tolerance,
                    ok,
                    verdict,
                }
            })
            .collect();
        Comparison {
            target: self.name.clone(),
            passed: results.iter().filter(|r| r.ok).count(),
            of: results.len(),
            results,
        }
    }
}

fn trim(x: f32) -> String {
    let s = format!("{x:.2}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_targets_parse_and_name_known_metrics() {
        let known = [
            "peak_dbfs",
            "rms_dbfs",
            "centroid_hz",
            "tilt_db_per_octave",
            "noisiness",
            "low_correlation",
            "mid_correlation",
            "high_correlation",
            "mid_balance_db",
            "attack_ms",
            "change_db",
            "brightness_start_hz",
            "brightness_mid_hz",
            "brightness_end_hz",
            "harmonic_2_db",
            "harmonic_3_db",
            "harmonic_4_db",
            "harmonic_5_db",
            "harmonic_6_db",
            "harmonic_7_db",
            "harmonic_8_db",
            "tuning_cents",
            "wobble_cents",
            "glide_ms",
        ];
        let all = Target::factory();
        assert_eq!(all.len(), 4);
        for t in &all {
            assert!(t.phrase.is_some() && t.bpm.is_some(), "{}", t.name);
            for m in t.criteria.keys() {
                assert!(
                    known.contains(&m.as_str()),
                    "{}: unknown metric {m}",
                    t.name
                );
            }
        }
        assert!(Target::find("nope").unwrap_err().contains("deep_moog_bass"));
    }
}
