//! Claude's ears: audition a part offline, listen to the output, analyse a
//! reference file. All three return a [`moodscore_ear::Report`].

use std::path::{Component, Path, PathBuf};

use moodscore_director::{Conductor, Curve, TempoMap};
use moodscore_ear::{analyze, Audio, NoteSpan, Options, Report, Target};
use moodscore_engine::{render_part_offline, synth_params::P};

use crate::notes::{self, Step};

/// Longest audition, in seconds (phrase plus tail).
const MAX_AUDITION: f64 = 40.0;
/// Longest stretch of a file analysed at once.
pub const MAX_FILE_SECONDS: f64 = 60.0;

/// Where reference WAVs live: `MOODSCORE_REFERENCES`; else `referencias/`
/// at the root of the checkout the binary was built in; else the platform's
/// data folder (`…/Moodscore/references`).
pub fn references_dir() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("MOODSCORE_REFERENCES") {
        return Some(PathBuf::from(d));
    }
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(4) {
            if dir.join("Cargo.toml").is_file() && dir.join("crates").is_dir() {
                return Some(dir.join("referencias"));
            }
        }
    }
    notes::user_dir().and_then(|p| p.parent().map(|d| d.join("references")))
}

/// A file inside the references folder: a plain relative path, no `..`.
pub fn reference_file(name: &str) -> Result<PathBuf, String> {
    let dir = references_dir().ok_or("no references folder (set MOODSCORE_REFERENCES)")?;
    let rel = Path::new(name.trim());
    let plain = rel.components().all(|c| matches!(c, Component::Normal(_)));
    if !plain || name.trim().is_empty() {
        return Err(format!(
            "give a file name inside the references folder ({}), not a path",
            dir.display()
        ));
    }
    if !rel
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("wav"))
    {
        return Err("only WAV files can be analysed".into());
    }
    let path = dir.join(rel);
    if !path.is_file() {
        let mut have: Vec<String> = std::fs::read_dir(&dir)
            .map(|rd| {
                rd.flatten()
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .filter(|n| n.to_ascii_lowercase().ends_with(".wav"))
                    .collect()
            })
            .unwrap_or_default();
        have.sort();
        have.truncate(30);
        return Err(format!(
            "no '{name}' in {}; there are: {}",
            dir.display(),
            if have.is_empty() {
                "no WAV files".into()
            } else {
                have.join(", ")
            }
        ));
    }
    Ok(path)
}

/// When each step of a phrase sounds, for measuring its notes one by one.
pub fn spans(tempo: &TempoMap, start: f64, steps: &[Step]) -> (Vec<NoteSpan>, f64) {
    let qpb = tempo.meter().quarters_per_beat();
    let mut b = tempo.beat_at(start);
    let mut out = Vec::new();
    for s in steps {
        let q = s.beats as f64 * qpb;
        out.push(NoteSpan {
            start: tempo.time_at(b),
            end: tempo.time_at(b + q * s.gate as f64),
            notes: s.notes.clone(),
        });
        b += q;
    }
    (out, tempo.time_at(b))
}

pub struct AuditionRequest {
    pub part: usize,
    pub phrase: Option<String>,
    pub bpm: Option<f64>,
    pub room: bool,
    pub target: Option<Target>,
}

/// Render `part`'s current sound playing a phrase, offline, and measure it.
pub fn audition(c: &Conductor, req: AuditionRequest) -> Result<(Report, String, f64), String> {
    let phrase = req
        .phrase
        .clone()
        .or_else(|| req.target.as_ref().and_then(|t| t.phrase.clone()))
        .unwrap_or_else(|| "A3:2".into());
    let steps = notes::parse_phrase(&phrase, 0.8, 0.9)?;
    let now_tempo = c.tempo();
    let now = c.now();
    let bpm = req
        .bpm
        .or_else(|| req.target.as_ref().and_then(|t| t.bpm.map(f64::from)))
        .unwrap_or_else(|| now_tempo.bpm_at(now));
    if !(20.0..=300.0).contains(&bpm) {
        return Err("bpm must be between 20 and 300".into());
    }
    let tempo = TempoMap::default().changed(
        0.0,
        Some(bpm),
        Some(now_tempo.meter()),
        0.0,
        Curve::Linear,
        true,
    );
    const LEAD_IN: f64 = 0.05;
    let events = notes::schedule(&tempo, LEAD_IN, &steps, 1);
    let (spans, end) = spans(&tempo, LEAD_IN, &steps);

    let patch = notes::current_patch(c, req.part);
    let g = |p: P| patch.0[p as usize] as f64;
    // Long enough for the release, the echoes and the room to die away.
    let mut tail = g(P::AmpRelease) * 1.5 + 0.3;
    if g(P::DelayMix) > 0.0 {
        tail += 1.5;
    }
    if req.room {
        tail += 2.0;
    }
    let seconds = (end + tail.min(8.0)).min(MAX_AUDITION);
    if end > MAX_AUDITION {
        return Err(format!(
            "that phrase lasts {end:.1} s; an audition can be at most {MAX_AUDITION} s"
        ));
    }
    let sr = c.engine().sample_rate();
    let room: Option<Vec<f32>> = req.room.then(|| {
        let e = c.engine();
        let n = e.dsp_param_count();
        let mut v = c.targets(0..n);
        for (id, p) in e.params()[..n].iter().enumerate() {
            if p.path.starts_with("drone_") && p.path.ends_with("/level") {
                v[id] = 0.0;
            }
        }
        v
    });
    let frame_events: Vec<(u64, moodscore_engine::NoteEvent)> = events
        .iter()
        .map(|(t, e)| ((t * sr as f64) as u64, *e))
        .collect();
    let (left, right) = render_part_offline(
        sr,
        &patch.0,
        room.as_deref(),
        &frame_events,
        (seconds * sr as f64) as usize,
        tempo.quarters_per_second(0.0) as f32,
    );
    let audio = Audio {
        sample_rate: sr,
        left,
        right,
    };
    let mut report = analyze(
        &audio,
        &Options {
            notes: spans,
            ..Options::default()
        },
    );
    if let Some(t) = &req.target {
        report.comparison = Some(t.compare(&report));
    }
    Ok((report, phrase, bpm))
}

/// Measure the last `seconds` of what has been playing.
pub fn listen(
    c: &Conductor,
    seconds: f64,
    pitch_range: Option<(f32, f32)>,
    target: Option<&Target>,
) -> Result<Report, String> {
    let (left, right) = c.recent_output(seconds);
    let sr = c.engine().sample_rate();
    if left.len() < sr as usize / 4 {
        return Err("nothing heard yet: the output starts being kept when the engine runs".into());
    }
    let mut opts = Options::default();
    if let Some(p) = pitch_range {
        opts.pitch_range = p;
    }
    let mut report = analyze(
        &Audio {
            sample_rate: sr,
            left,
            right,
        },
        &opts,
    );
    if let Some(t) = target {
        report.comparison = Some(t.compare(&report));
    }
    Ok(report)
}

pub struct FileRequest {
    pub file: String,
    pub from: Option<f64>,
    pub to: Option<f64>,
    pub band: Option<(f32, f32)>,
    pub pitch_range: Option<(f32, f32)>,
    pub target: Option<Target>,
}

/// Measure (part of) a reference WAV.
pub fn analyze_file(req: FileRequest) -> Result<Report, String> {
    let path = reference_file(&req.file)?;
    let audio = Audio::read_wav(&path)?;
    let from = req.from.unwrap_or(0.0).max(0.0);
    let to = req.to.unwrap_or(audio.seconds()).min(audio.seconds());
    if to <= from {
        return Err(format!(
            "nothing between {from} and {to} s (the file lasts {:.2} s)",
            audio.seconds()
        ));
    }
    if to - from > MAX_FILE_SECONDS {
        return Err(format!(
            "analyse at most {MAX_FILE_SECONDS} s at a time (use from and to)"
        ));
    }
    let mut a = audio.slice(from, to);
    if let Some((lo, hi)) = req.band {
        if !(lo >= 0.0 && hi > lo) {
            return Err("band needs [low, high] in Hz, low below high".into());
        }
        a = a.band(Some(lo).filter(|v| *v > 0.0), Some(hi));
    }
    let mut opts = Options::default();
    if let Some((lo, hi)) = req.pitch_range {
        if !(lo >= 20.0 && hi > lo && hi <= 5000.0) {
            return Err("pitch_range needs [low, high] in Hz between 20 and 5000".into());
        }
        opts.pitch_range = (lo, hi);
    }
    let mut report = analyze(&a, &opts);
    if let Some(t) = &req.target {
        report.comparison = Some(t.compare(&report));
    }
    Ok(report)
}
