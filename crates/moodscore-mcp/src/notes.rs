//! The note parts as the agent sees them: presets, patches and phrases.

use std::path::PathBuf;

use moodscore_director::harmony::parse_note;
use moodscore_director::{Conductor, TempoMap};
use moodscore_engine::{part_prefix, synth_params, NoteEvent, PARTS};
use moodscore_synth::preset::{self, Preset};
use moodscore_synth::Patch;
use serde::Serialize;
use serde_json::{Map, Value};

/// Where saved presets go: `MOODSCORE_PRESETS`, or the platform's data folder.
pub fn user_dir() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("MOODSCORE_PRESETS") {
        return Some(PathBuf::from(d));
    }
    if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| PathBuf::from(a).join("Moodscore").join("presets"))
    } else if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| {
            PathBuf::from(h)
                .join("Library/Application Support/Moodscore")
                .join("presets")
        })
    } else {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
            .map(|d| d.join("moodscore").join("presets"))
    }
}

/// Factory presets, then the user's (a user preset can't shadow a factory one).
pub fn all_presets() -> Vec<(Preset, bool)> {
    let mut all: Vec<(Preset, bool)> = Preset::factory().into_iter().map(|p| (p, true)).collect();
    if let Some(dir) = user_dir() {
        if let Ok(entries) = std::fs::read_dir(dir) {
            let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            files.sort();
            for f in files {
                if f.extension().is_some_and(|e| e == "json") {
                    if let Ok(p) = std::fs::read_to_string(&f)
                        .map_err(|e| e.to_string())
                        .and_then(|s| Preset::parse(&s))
                    {
                        if !all
                            .iter()
                            .any(|(q, _)| q.name.eq_ignore_ascii_case(&p.name))
                        {
                            all.push((p, false));
                        }
                    }
                }
            }
        }
    }
    all
}

pub fn find_preset(name: &str) -> Result<Preset, String> {
    let all = all_presets();
    all.iter()
        .find(|(p, _)| p.name.eq_ignore_ascii_case(name.trim()))
        .map(|(p, _)| p.clone())
        .ok_or_else(|| {
            let names: Vec<&str> = all.iter().map(|(p, _)| p.name.as_str()).collect();
            format!("no preset called '{name}'; there are: {}", names.join(", "))
        })
}

/// "1", "2", "note_1", "note_2" → part index.
pub fn part_index(arg: Option<&Value>) -> Result<usize, String> {
    let bad = || "part must be 1 or 2 (or \"note_1\", \"note_2\")".to_string();
    let n = match arg {
        None | Some(Value::Null) => 1,
        Some(Value::Number(n)) => n.as_u64().ok_or_else(bad)? as usize,
        Some(Value::String(s)) => s
            .trim()
            .trim_start_matches("note_")
            .parse::<usize>()
            .map_err(|_| bad())?,
        _ => return Err(bad()),
    };
    if !(1..=PARTS).contains(&n) {
        return Err(bad());
    }
    Ok(n - 1)
}

/// What the conductor holds for a part, as a patch.
pub fn current_patch(c: &Conductor, part: usize) -> Patch {
    let e = c.engine();
    let ids: Vec<usize> = synth_params::PARAMS
        .iter()
        .map(|d| e.part_param_id(part, d.path).unwrap_or(0))
        .collect();
    Patch(c.targets(ids))
}

/// The changes, as (full path, value) pairs for the conductor.
pub fn to_changes(part: usize, values: &[(String, f32)]) -> Vec<(String, f32)> {
    let prefix = part_prefix(part);
    values
        .iter()
        .map(|(p, v)| (format!("{prefix}/{p}"), *v))
        .collect()
}

/// Parse a `set` object: path → number or choice name.
pub fn parse_set(set: &Map<String, Value>) -> Result<Vec<(String, f32)>, String> {
    set.iter()
        .map(|(k, v)| {
            let path = k.trim();
            preset::parse_value(path, v).map(|x| (path.to_string(), x))
        })
        .collect()
}

#[derive(Serialize)]
pub struct PartReport {
    pub part: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// True if the patch differs from the preset loaded.
    pub modified: bool,
    pub sounding: usize,
    pub description: String,
    /// Every parameter that differs from the default patch.
    pub params: Map<String, Value>,
}

/// Brief per-part status for every report.
#[derive(Serialize)]
pub struct PartStatus {
    pub part: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    pub modified: bool,
    pub sounding: usize,
    /// Notes scheduled but not yet played, and seconds until the last one.
    #[serde(skip_serializing_if = "is_zero")]
    pub waiting: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrase_ends_in: Option<f64>,
    /// The sequence playing, and the next one if it waits for its bar.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sequences: Vec<moodscore_director::SequenceState>,
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}

pub fn status(c: &Conductor, part: usize, loaded: &Option<(String, Patch)>) -> PartStatus {
    let patch = current_patch(c, part);
    let (waiting, last) = c.pending_notes(part);
    PartStatus {
        part: part_prefix(part),
        preset: loaded.as_ref().map(|(n, _)| n.clone()),
        modified: loaded.as_ref().is_some_and(|(_, p)| differs(p, &patch)),
        sounding: c.engine().active_notes(part),
        waiting,
        phrase_ends_in: last.map(|t| ((t - c.now()).max(0.0) * 100.0).round() / 100.0),
        sequences: c.sequences_of(part),
    }
}

pub fn report(c: &Conductor, part: usize, loaded: &Option<(String, Patch)>) -> PartReport {
    let patch = current_patch(c, part);
    let s = status(c, part, loaded);
    PartReport {
        part: s.part,
        preset: s.preset,
        modified: s.modified,
        sounding: s.sounding,
        description: preset::describe(&patch),
        params: preset::changed(&patch),
    }
}

fn differs(a: &Patch, b: &Patch) -> bool {
    a.0.iter()
        .zip(&b.0)
        .any(|(x, y)| (x - y).abs() > 1e-4 * x.abs().max(1.0))
}

/// A preset name that is safe as a file name.
pub fn check_name(name: &str) -> Result<String, String> {
    let n = name.trim().to_ascii_lowercase().replace(' ', "_");
    if n.is_empty()
        || n.len() > 40
        || !n
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err("a preset name uses letters, digits, spaces, _ or - (up to 40)".into());
    }
    Ok(n)
}

/// One step of a phrase.
pub struct Step {
    /// MIDI notes (empty = rest).
    pub notes: Vec<u8>,
    /// Beats of the metre.
    pub beats: f32,
    pub velocity: f32,
    pub gate: f32,
}

/// "A3", "C#4 E4 G#4", "60", "rest" → MIDI notes.
pub fn parse_notes(s: &str) -> Result<Vec<u8>, String> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("rest") || t == "-" {
        return Ok(Vec::new());
    }
    t.split([' ', ',', '+'])
        .filter(|w| !w.is_empty())
        .map(|w| {
            if let Ok(m) = w.parse::<u8>() {
                return if m <= 127 {
                    Ok(m)
                } else {
                    Err(format!("note {m} is out of range"))
                };
            }
            let n = parse_note(w).map_err(|e| e.to_string())?;
            n.midi
                .ok_or_else(|| format!("'{w}': give the octave too, e.g. A3 (C4 is middle C)"))
        })
        .collect()
}

/// Parse the short form of a phrase. Tokens, separated by spaces:
/// `A3:0.5` (note:beats), `A3+C4+E4:2` (chord), `r:1` or `-:1` (rest),
/// `D5:1@0.9` (velocity), `F#5:1.5~` (held into the next step). A missing
/// length repeats the previous one (1 at the start); `|` is ignored. Beats
/// may be written as fractions (`1/2`, `3/4`).
pub fn parse_phrase(s: &str, velocity: f32, gate: f32) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    let mut beats = 1.0f32;
    for (i, raw) in s.split_whitespace().enumerate() {
        if raw.chars().all(|c| c == '|') {
            continue;
        }
        let at = |e: String| format!("token {} ('{raw}'): {e}", i + 1);
        let mut tok = raw;
        let held = tok.ends_with('~');
        tok = tok.trim_end_matches('~');
        let (tok, vel) = match tok.split_once('@') {
            Some((t, v)) => (
                t,
                v.parse::<f32>()
                    .map_err(|_| at(format!("'{v}' is not a velocity (0 to 1)")))?,
            ),
            None => (tok, velocity),
        };
        let (pitches, len) = match tok.split_once(':') {
            Some((p, l)) => (p, Some(l)),
            None => (tok, None),
        };
        if let Some(l) = len {
            beats = parse_beats(l).ok_or_else(|| at(format!("'{l}' is not a length in beats")))?;
        }
        let notes = if matches!(pitches, "r" | "R" | "-" | "rest") {
            Vec::new()
        } else {
            let n = parse_notes(&pitches.replace('+', " ")).map_err(at)?;
            if n.is_empty() {
                return Err(at("no note".into()));
            }
            if n.len() > 8 {
                return Err(at("at most 8 notes at once".into()));
            }
            n
        };
        steps.push(Step {
            notes,
            beats,
            velocity: vel,
            gate: if held { gate.max(1.05) } else { gate },
        });
    }
    if steps.is_empty() {
        return Err("the phrase has no notes or rests".into());
    }
    Ok(steps)
}

/// `0.5`, `2`, `1/2`, `3/4`.
fn parse_beats(s: &str) -> Option<f32> {
    match s.split_once('/') {
        Some((a, b)) => {
            let (a, b) = (a.trim().parse::<f32>().ok()?, b.trim().parse::<f32>().ok()?);
            (b != 0.0).then(|| a / b)
        }
        None => s.trim().parse().ok(),
    }
}

/// Turn a phrase into timed note events on the director's clock, starting
/// at `start` (seconds), repeated `repeat` times, following `tempo`.
pub fn schedule(
    tempo: &TempoMap,
    start: f64,
    steps: &[Step],
    repeat: u32,
) -> Vec<(f64, NoteEvent)> {
    let qpb = tempo.meter().quarters_per_beat();
    let mut b = tempo.beat_at(start);
    // (time, order, event): offs sort before ons at the same instant.
    let mut out: Vec<(f64, u8, NoteEvent)> = Vec::new();
    for _ in 0..repeat {
        for s in steps {
            let q = s.beats as f64 * qpb;
            let on_t = tempo.time_at(b);
            let off_t = tempo.time_at(b + q * s.gate as f64);
            for &note in &s.notes {
                out.push((
                    on_t,
                    1,
                    NoteEvent::NoteOn {
                        note,
                        velocity: s.velocity,
                    },
                ));
                out.push((off_t, 0, NoteEvent::NoteOff { note }));
            }
            b += q;
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    out.into_iter().map(|(t, _, e)| (t, e)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_phrases_parse() {
        let s = parse_phrase("B3:0.5 D4 G4:1 | A3+C#4+E4:2@0.6 r:1/2 F#5:1.5~", 0.8, 0.9).unwrap();
        let beats: Vec<f32> = s.iter().map(|s| s.beats).collect();
        assert_eq!(beats, [0.5, 0.5, 1.0, 2.0, 0.5, 1.5]);
        assert_eq!(s[0].notes, [59]);
        assert_eq!(s[3].notes, [57, 61, 64]);
        assert_eq!(s[3].velocity, 0.6);
        assert!(s[4].notes.is_empty());
        assert_eq!(s[5].gate, 1.05);
        assert_eq!(s[0].gate, 0.9);
        for bad in ["A3:x", "H3:1", "A:1", "A3:1@loud", "|"] {
            assert!(parse_phrase(bad, 0.8, 0.9).is_err(), "{bad}");
        }
    }
}
