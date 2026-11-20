//! Moodscore's ears.
//!
//! Measures a sound the way a sound designer listens to it, and reports
//! numbers an agent can iterate on: level; brightness, tilt, noisiness and
//! the levels of the harmonics; how each note starts, evolves and ends
//! (attack, change of level, brightness at the start, middle and end);
//! pitch (tuning, wobble, glides between notes); and stereo width and
//! balance in the lows, mids and highs. A [`Target`] turns that into a
//! pass/fail comparison with tolerances.
//!
//! It works on any audio: a part rendered offline (with the notes it was
//! given), the live output, or a reference file. Nothing here runs on the
//! audio thread.

pub mod filters;
pub mod pitch;
pub mod spectrum;
pub mod target;

use std::collections::BTreeMap;

use serde::Serialize;

pub use target::{Comparison, Target};

/// Stereo audio to analyse.
#[derive(Clone, Debug)]
pub struct Audio {
    pub sample_rate: u32,
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

impl Audio {
    pub fn seconds(&self) -> f64 {
        self.left.len() as f64 / self.sample_rate as f64
    }

    pub fn mono(&self) -> Vec<f32> {
        self.left
            .iter()
            .zip(&self.right)
            .map(|(l, r)| 0.5 * (l + r))
            .collect()
    }

    /// The part between `from` and `to` seconds.
    pub fn slice(&self, from: f64, to: f64) -> Audio {
        let sr = self.sample_rate as f64;
        let a = ((from.max(0.0) * sr) as usize).min(self.left.len());
        let b = ((to * sr) as usize).clamp(a, self.left.len());
        Audio {
            sample_rate: self.sample_rate,
            left: self.left[a..b].to_vec(),
            right: self.right[a..b].to_vec(),
        }
    }

    /// Only the band `lo..hi` Hz (either end optional).
    pub fn band(&self, lo: Option<f32>, hi: Option<f32>) -> Audio {
        Audio {
            sample_rate: self.sample_rate,
            left: filters::band(&self.left, self.sample_rate, lo, hi),
            right: filters::band(&self.right, self.sample_rate, lo, hi),
        }
    }

    /// Read a WAV file (any common bit depth; mono is duplicated).
    pub fn read_wav(path: &std::path::Path) -> Result<Audio, String> {
        let mut r = hound::WavReader::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let spec = r.spec();
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => r
                .samples::<f32>()
                .collect::<Result<_, _>>()
                .map_err(|e| e.to_string())?,
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1u64 << (spec.bits_per_sample - 1)) as f32;
                r.samples::<i32>()
                    .map(|s| s.map(|v| v as f32 * scale))
                    .collect::<Result<_, _>>()
                    .map_err(|e| e.to_string())?
            }
        };
        let ch = spec.channels.max(1) as usize;
        let frames = samples.len() / ch;
        let mut left = Vec::with_capacity(frames);
        let mut right = Vec::with_capacity(frames);
        for f in 0..frames {
            left.push(samples[f * ch]);
            right.push(samples[f * ch + if ch > 1 { 1 } else { 0 }]);
        }
        Ok(Audio {
            sample_rate: spec.sample_rate,
            left,
            right,
        })
    }
}

/// A note (or chord) that was played, so it can be measured on its own.
#[derive(Clone, Debug)]
pub struct NoteSpan {
    /// Seconds into the audio.
    pub start: f64,
    /// When the key was released.
    pub end: f64,
    /// MIDI notes (several for a chord; empty for a rest).
    pub notes: Vec<u8>,
}

/// What to listen for.
#[derive(Clone, Debug)]
pub struct Options {
    /// Where to look for a pitch when the notes are not known (Hz).
    pub pitch_range: (f32, f32),
    /// The notes played, if known. Without them the whole audio is measured
    /// as one event, and pitch is tracked.
    pub notes: Vec<NoteSpan>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            pitch_range: (30.0, 2000.0),
            notes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Report {
    pub seconds: f32,
    /// The sound in words.
    pub summary: String,
    pub level: Level,
    pub spectrum: Spectrum,
    pub stereo: Stereo,
    /// Brightness and level in eight equal stretches, start to end.
    pub over_time: OverTime,
    /// One entry per note played (when the notes are known).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<NoteReport>,
    /// The pitch heard (when the notes are not known).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<PitchReport>,
    /// The headline numbers, flat, for comparing with a target.
    pub metrics: BTreeMap<String, f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<Comparison>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Level {
    pub peak_dbfs: f32,
    pub rms_dbfs: f32,
    pub crest_db: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Spectrum {
    /// Median spectral centroid: higher is brighter.
    pub centroid_hz: f32,
    /// Share of energy: sub < 60 Hz, bass < 250, low_mid < 1 k, mid < 4 k, high above.
    pub bands_percent: BTreeMap<&'static str, f32>,
    /// Slope of the octave bands from 63 Hz to 8 kHz: a saw is about −6, darker is steeper.
    pub tilt_db_per_octave: f32,
    /// 0 for a pure tone, toward 1 for noise.
    pub noisiness: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Stereo {
    /// Below 150 Hz.
    pub low: Option<BandStereo>,
    /// 150 Hz to 3 kHz.
    pub mid: Option<BandStereo>,
    /// 3 to 12 kHz.
    pub high: Option<BandStereo>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BandStereo {
    /// 1 is mono, 0 is fully decorrelated (very wide), below 0 out of phase.
    pub correlation: f32,
    /// Positive: louder on the left.
    pub balance_db: f32,
}

#[derive(Clone, Debug, Serialize)]
pub struct OverTime {
    pub brightness_hz: Vec<f32>,
    pub level_db: Vec<f32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NoteReport {
    pub notes: String,
    pub start_s: f32,
    /// From the note's start to 90 % of its peak level.
    pub attack_ms: f32,
    pub peak_dbfs: f32,
    /// Level at the end of the note against its peak (negative: it decays).
    pub change_db: f32,
    /// Spectral centroid at 10 %, 50 % and 90 % of the note.
    pub brightness_hz: [f32; 3],
    /// Harmonics 2 to 8 against the fundamental (single notes only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmonics_db: Option<Vec<f32>>,
    /// Median distance from the written pitch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_cents: Option<f32>,
    /// How much the pitch moves while held (standard deviation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wobble_cents: Option<f32>,
    /// Time to arrive from the previous note (legato patches).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glide_ms: Option<f32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PitchReport {
    /// Held notes in order, with when they start: "1.25s F4".
    pub notes: Vec<String>,
    /// The longest held note and its harmonics 2 to 8 against the fundamental.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmonics_db: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wobble_cents: Option<f32>,
    /// Median time between held notes when the pitch slides between them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glide_ms: Option<f32>,
}

/// Decibels, floored at -100 (silence).
fn db(x: f32) -> f32 {
    (20.0 * x.max(1e-9).log10()).max(-100.0)
}

/// A report as JSON, with every number written as short as its precision
/// allows (0.3, not 0.30000001192092896).
pub fn to_json(report: &Report) -> serde_json::Value {
    fn tidy(v: &mut serde_json::Value) {
        match v {
            serde_json::Value::Number(n) if n.is_f64() => {
                let f = n.as_f64().unwrap_or(0.0) as f32;
                if let Some(t) = f
                    .to_string()
                    .parse::<f64>()
                    .ok()
                    .and_then(serde_json::Number::from_f64)
                {
                    *n = t;
                }
            }
            serde_json::Value::Array(a) => a.iter_mut().for_each(tidy),
            serde_json::Value::Object(o) => o.values_mut().for_each(tidy),
            _ => {}
        }
    }
    let mut v = serde_json::to_value(report).unwrap_or_default();
    tidy(&mut v);
    v
}

fn round(x: f32, step: f32) -> f32 {
    (x / step).round() * step
}

fn rms(x: &[f32]) -> f32 {
    if x.is_empty() {
        return 0.0;
    }
    (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt()
}

fn mean(v: &[f32]) -> Option<f32> {
    let v: Vec<f32> = v.iter().cloned().filter(|x| x.is_finite()).collect();
    (!v.is_empty()).then(|| v.iter().sum::<f32>() / v.len() as f32)
}

fn median(mut v: Vec<f32>) -> Option<f32> {
    v.retain(|x| x.is_finite());
    v.sort_by(|a, b| a.total_cmp(b));
    (!v.is_empty()).then(|| v[v.len() / 2])
}

fn stereo_band(a: &Audio, lo: Option<f32>, hi: Option<f32>) -> Option<BandStereo> {
    let l = filters::band(&a.left, a.sample_rate, lo, hi);
    let r = filters::band(&a.right, a.sample_rate, lo, hi);
    let (mut ll, mut rr, mut lr) = (0.0f64, 0.0f64, 0.0f64);
    for (x, y) in l.iter().zip(&r) {
        ll += (*x as f64) * (*x as f64);
        rr += (*y as f64) * (*y as f64);
        lr += (*x as f64) * (*y as f64);
    }
    // Ignore bands with next to nothing in them (below -70 dBFS RMS).
    let floor = 1e-7 * l.len().max(1) as f64;
    if ll + rr < floor {
        return None;
    }
    Some(BandStereo {
        correlation: round((lr / (ll * rr).sqrt().max(1e-30)) as f32, 0.01),
        balance_db: round(10.0 * ((ll + 1e-30) / (rr + 1e-30)).log10() as f32, 0.1),
    })
}

/// Measure `audio`.
pub fn analyze(audio: &Audio, opts: &Options) -> Report {
    let sr = audio.sample_rate;
    let mono = audio.mono();
    let dur = audio.seconds();
    let peak = audio
        .left
        .iter()
        .chain(&audio.right)
        .fold(0.0f32, |m, s| m.max(s.abs()));
    let level_rms = rms(&mono);

    // Frames: centroid, flatness, level.
    let frames = spectrum::frames(&mono, sr);
    let hop_s = spectrum::HOP as f64 / sr as f64;
    let frame_time = |i: usize| (i as f64 * hop_s) + spectrum::FRAME as f64 / 2.0 / sr as f64;
    let loudest = frames.iter().map(|f| f.2).fold(0.0, f32::max);
    let audible = |f: &(f32, f32, f32)| f.2 > loudest * 10f32.powf(-50.0 / 20.0);
    let centroid =
        median(frames.iter().filter(|f| audible(f)).map(|f| f.0).collect()).unwrap_or(0.0);
    let noisiness =
        median(frames.iter().filter(|f| audible(f)).map(|f| f.1).collect()).unwrap_or(0.0);

    // Band shares and tilt from the long-term spectrum.
    let (mags, bin) = spectrum::magnitude(&mono, sr, 1 << 15);
    let energy = |lo: f32, hi: f32| -> f64 {
        let a = (lo / bin) as usize;
        let b = ((hi / bin) as usize).min(mags.len());
        mags[a.min(b)..b].iter().map(|m| (*m as f64).powi(2)).sum()
    };
    let total = energy(20.0, 20_000.0).max(1e-30);
    let mut bands_percent = BTreeMap::new();
    for (name, lo, hi) in [
        ("1_sub", 20.0, 60.0),
        ("2_bass", 60.0, 250.0),
        ("3_low_mid", 250.0, 1000.0),
        ("4_mid", 1000.0, 4000.0),
        ("5_high", 4000.0, 20_000.0),
    ] {
        bands_percent.insert(name, round((100.0 * energy(lo, hi) / total) as f32, 0.1));
    }
    let octaves: Vec<(f32, f32)> = (0..8)
        .map(|k| {
            let c = 62.5 * 2f32.powi(k);
            (
                k as f32,
                10.0 * (energy(c / 1.414, c * 1.414) as f32 + 1e-20).log10(),
            )
        })
        .collect();
    let tilt = {
        let n = octaves.len() as f32;
        let mx = octaves.iter().map(|p| p.0).sum::<f32>() / n;
        let my = octaves.iter().map(|p| p.1).sum::<f32>() / n;
        let num: f32 = octaves.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
        let den: f32 = octaves.iter().map(|p| (p.0 - mx).powi(2)).sum();
        num / den.max(1e-9)
    };

    let stereo = Stereo {
        low: stereo_band(audio, Some(20.0), Some(150.0)),
        mid: stereo_band(audio, Some(150.0), Some(3000.0)),
        high: stereo_band(audio, Some(3000.0), Some(12_000.0)),
    };

    // Over time: eight stretches.
    let mut over_time = OverTime {
        brightness_hz: Vec::new(),
        level_db: Vec::new(),
    };
    if !frames.is_empty() {
        let per = frames.len().div_ceil(8);
        for chunk in frames.chunks(per) {
            let w: f32 = chunk.iter().map(|f| f.2).sum();
            let c = chunk.iter().map(|f| f.0 * f.2).sum::<f32>() / w.max(1e-12);
            over_time.brightness_hz.push(round(c, 10.0));
            over_time.level_db.push(round(
                db(rms(&chunk.iter().map(|f| f.2).collect::<Vec<_>>())),
                0.5,
            ));
        }
    }

    // Brightness at a moment: frames around time t, weighted by level.
    let brightness_at = |t: f64| -> f32 {
        let idx: Vec<usize> = (0..frames.len())
            .filter(|&i| (frame_time(i) - t).abs() <= hop_s * 1.5)
            .collect();
        let w: f32 = idx.iter().map(|&i| frames[i].2).sum();
        if w <= 0.0 {
            return f32::NAN;
        }
        idx.iter().map(|&i| frames[i].0 * frames[i].2).sum::<f32>() / w
    };

    // Envelope in 5 ms steps.
    let step = (sr as f64 * 0.005) as usize;
    let env: Vec<f32> = mono.chunks(step.max(1)).map(rms).collect();
    let env_at = |t: f64| ((t / 0.005) as usize).min(env.len().saturating_sub(1));

    let mut notes = Vec::new();
    let mut pitch_report = None;
    let spans: Vec<NoteSpan> = if opts.notes.is_empty() {
        vec![NoteSpan {
            start: 0.0,
            end: dur,
            notes: Vec::new(),
        }]
    } else {
        opts.notes.clone()
    };
    let mut prev_single: Option<u8> = None;
    for (i, span) in spans.iter().enumerate() {
        if !opts.notes.is_empty() && span.notes.is_empty() {
            continue;
        }
        let next_start = spans
            .iter()
            .skip(i + 1)
            .find(|s| !s.notes.is_empty())
            .map_or(dur, |s| s.start);
        let end = span.end.min(next_start).min(dur);
        if end <= span.start + 0.01 || env.is_empty() {
            continue;
        }
        let (a, b) = (env_at(span.start), env_at(end).max(env_at(span.start) + 1));
        let seg = &env[a..b.min(env.len())];
        let (pk_i, pk) = seg
            .iter()
            .enumerate()
            .fold((0, 0.0f32), |m, (k, &v)| if v > m.1 { (k, v) } else { m });
        let attack = seg.iter().position(|&v| v >= 0.9 * pk).unwrap_or(pk_i);
        let tail = &seg[seg.len().saturating_sub(10)..];
        let change = db(rms(tail)) - db(pk);
        let len = end - span.start;
        let bright = [0.1, 0.5, 0.9].map(|f| round(brightness_at(span.start + f * len), 10.0));

        let single = (span.notes.len() == 1).then(|| span.notes[0]);
        let (mut harmonics_db, mut tuning, mut wobble, mut glide) = (None, None, None, None);
        if let Some(n) = single {
            let f0 = 440.0 * 2f32.powf((n as f32 - 69.0) / 12.0);
            let body = audio.slice(span.start + 0.2 * len, end);
            let h = spectrum::harmonics(&body.mono(), sr, f0, 8);
            harmonics_db = Some(h[1..].iter().map(|v| round(*v, 0.5)).collect());
            let seg_audio = audio.slice(span.start, end);
            let t = pitch::track(&seg_audio.mono(), sr, f0 / 1.6, f0 * 1.6, 128.min(step));
            let target = n as f32;
            let devs: Vec<f32> = t.hz[t.hz.len() / 5..]
                .iter()
                .flatten()
                .map(|f| (pitch::midi(*f) - target) * 100.0)
                .filter(|d| d.abs() < 50.0)
                .collect();
            if let Some(m) = median(devs.clone()) {
                tuning = Some(round(m, 0.1));
                let mu = mean(&devs).unwrap_or(0.0);
                let var = devs.iter().map(|d| (d - mu).powi(2)).sum::<f32>() / devs.len() as f32;
                wobble = Some(round(var.sqrt(), 0.1));
            }
            if prev_single.is_some_and(|p| p != n) {
                glide =
                    t.hz.iter()
                        .position(|f| f.is_some_and(|f| (pitch::midi(f) - target).abs() <= 0.35))
                        .map(|k| round((k as f64 * t.hop_seconds * 1000.0) as f32, 1.0));
            }
        }
        prev_single = single;
        notes.push(NoteReport {
            notes: if span.notes.is_empty() {
                "(whole)".into()
            } else {
                span.notes
                    .iter()
                    .map(|n| pitch::note_name(*n as f32))
                    .collect::<Vec<_>>()
                    .join("+")
            },
            start_s: round(span.start as f32, 0.001),
            attack_ms: round(attack as f32 * 5.0, 1.0),
            peak_dbfs: round(db(pk), 0.1),
            change_db: round(change, 0.1),
            brightness_hz: bright,
            harmonics_db,
            tuning_cents: tuning,
            wobble_cents: wobble,
            glide_ms: glide,
        });
    }

    if opts.notes.is_empty() {
        let (lo, hi) = opts.pitch_range;
        // Track on a band around the range so other parts of a mix stay out.
        let banded = filters::band(&mono, sr, Some(lo * 0.7), Some(hi * 2.0));
        let t = pitch::track(&banded, sr, lo, hi, (sr as f64 * 0.005) as usize);
        let held = pitch::held_notes(&t, 0.06);
        if !held.is_empty() {
            let longest = held
                .iter()
                .max_by(|a, b| (a.end - a.start).total_cmp(&(b.end - b.start)))
                .cloned()
                .unwrap();
            let f0 = 440.0 * 2f32.powf((longest.note as f32 + longest.cents / 100.0 - 69.0) / 12.0);
            let body = audio.slice(longest.start, longest.end);
            let h = spectrum::harmonics(&body.mono(), sr, f0, 8);
            // A "harmonic" far above the fundamental means the fundamental
            // isn't really there (filtered out, or another part): don't report.
            let h_ok = h[1..].iter().all(|v| *v < 6.0);
            let glides = pitch::glides(&t, &held);
            pitch_report = Some(PitchReport {
                notes: held
                    .iter()
                    .take(48)
                    .map(|h| format!("{:.2}s {}", h.start, pitch::note_name(h.note as f32)))
                    .collect(),
                longest: Some(format!(
                    "{} ({:.2}s, {:+.0} cents)",
                    pitch::note_name(longest.note as f32),
                    longest.end - longest.start,
                    longest.cents
                )),
                harmonics_db: (h_ok && longest.end - longest.start >= 0.15)
                    .then(|| h[1..].iter().map(|v| round(*v, 0.5)).collect()),
                wobble_cents: median(held.iter().map(|h| h.wobble).collect())
                    .map(|v| round(v, 0.1)),
                glide_ms: median(glides.iter().map(|g| *g as f32 * 1000.0).collect())
                    .map(|v| round(v, 1.0)),
            });
        }
    }

    // Flat metrics.
    let mut m = BTreeMap::new();
    let mut put = |k: &str, v: Option<f32>| {
        if let Some(v) = v.filter(|v| v.is_finite()) {
            m.insert(k.to_string(), v);
        }
    };
    put("peak_dbfs", Some(round(db(peak), 0.1)));
    put("rms_dbfs", Some(round(db(level_rms), 0.1)));
    put("centroid_hz", Some(round(centroid, 10.0)));
    put("tilt_db_per_octave", Some(round(tilt, 0.1)));
    put("noisiness", Some(round(noisiness, 0.01)));
    put(
        "low_correlation",
        stereo.low.as_ref().map(|b| b.correlation),
    );
    put(
        "mid_correlation",
        stereo.mid.as_ref().map(|b| b.correlation),
    );
    put(
        "high_correlation",
        stereo.high.as_ref().map(|b| b.correlation),
    );
    put("mid_balance_db", stereo.mid.as_ref().map(|b| b.balance_db));
    let col = |f: &dyn Fn(&NoteReport) -> Option<f32>| {
        mean(&notes.iter().filter_map(f).collect::<Vec<_>>())
    };
    put(
        "attack_ms",
        col(&|n| Some(n.attack_ms)).map(|v| round(v, 1.0)),
    );
    put(
        "change_db",
        col(&|n| Some(n.change_db)).map(|v| round(v, 0.1)),
    );
    for (k, name) in [
        "brightness_start_hz",
        "brightness_mid_hz",
        "brightness_end_hz",
    ]
    .iter()
    .enumerate()
    {
        put(
            name,
            col(&|n| Some(n.brightness_hz[k])).map(|v| round(v, 10.0)),
        );
    }
    let harmonics: Option<Vec<f32>> = if opts.notes.is_empty() {
        pitch_report.as_ref().and_then(|p| p.harmonics_db.clone())
    } else {
        let all: Vec<&Vec<f32>> = notes
            .iter()
            .filter_map(|n| n.harmonics_db.as_ref())
            .collect();
        (!all.is_empty()).then(|| {
            (0..7)
                .map(|k| mean(&all.iter().map(|h| h[k]).collect::<Vec<_>>()).unwrap_or(f32::NAN))
                .collect()
        })
    };
    if let Some(h) = harmonics {
        for (k, v) in h.iter().enumerate() {
            put(&format!("harmonic_{}_db", k + 2), Some(round(*v, 0.5)));
        }
    }
    if opts.notes.is_empty() {
        let p = pitch_report.as_ref();
        put("wobble_cents", p.and_then(|p| p.wobble_cents));
        put("glide_ms", p.and_then(|p| p.glide_ms));
    } else {
        put(
            "tuning_cents",
            col(&|n| n.tuning_cents).map(|v| round(v, 0.1)),
        );
        put(
            "wobble_cents",
            col(&|n| n.wobble_cents).map(|v| round(v, 0.1)),
        );
        put("glide_ms", col(&|n| n.glide_ms).map(|v| round(v, 1.0)));
    }

    let level = Level {
        peak_dbfs: round(db(peak), 0.1),
        rms_dbfs: round(db(level_rms), 0.1),
        crest_db: round(db(peak) - db(level_rms), 0.1),
    };
    let summary = describe(&m, &level);
    let mut spectrum_out = Spectrum {
        centroid_hz: round(centroid, 10.0),
        bands_percent,
        tilt_db_per_octave: round(tilt, 0.1),
        noisiness: round(noisiness, 0.01),
    };
    if !spectrum_out.centroid_hz.is_finite() {
        spectrum_out.centroid_hz = 0.0;
    }
    Report {
        seconds: round(dur as f32, 0.01),
        summary,
        level,
        spectrum: spectrum_out,
        stereo,
        over_time,
        notes: if opts.notes.is_empty() {
            Vec::new()
        } else {
            notes
        },
        pitch: pitch_report,
        metrics: m,
        comparison: None,
    }
}

/// The headline numbers in a sentence.
fn describe(m: &BTreeMap<String, f32>, level: &Level) -> String {
    let mut words: Vec<String> = Vec::new();
    if level.rms_dbfs < -70.0 {
        return "silence".into();
    }
    if let Some(&c) = m.get("centroid_hz") {
        words.push(
            match c {
                c if c < 600.0 => "dark",
                c if c < 1500.0 => "warm",
                c if c < 3000.0 => "bright",
                _ => "very bright",
            }
            .to_string()
                + &format!(" ({c:.0} Hz)"),
        );
    }
    if let (Some(&h2), Some(&h5)) = (m.get("harmonic_2_db"), m.get("harmonic_5_db")) {
        words.push(
            if h2 < -12.0 && h5 < -30.0 {
                "mostly fundamental, round"
            } else if h5 > -18.0 {
                "rich in harmonics, buzzy"
            } else {
                "moderate harmonics"
            }
            .into(),
        );
    }
    if let Some(&n) = m.get("noisiness") {
        if n > 0.3 {
            words.push("noisy".into());
        }
    }
    if let Some(&a) = m.get("attack_ms") {
        words.push(
            match a {
                a if a < 12.0 => "sharp attack",
                a if a < 80.0 => "soft attack",
                a if a < 400.0 => "gentle attack",
                _ => "slow attack",
            }
            .into(),
        );
    }
    if let Some(&c) = m.get("change_db") {
        if c < -12.0 {
            words.push(format!("decays ({c:.0} dB over the note)"));
        } else if c > -3.0 {
            words.push("holds".into());
        }
    }
    if let (Some(&s), Some(&e)) = (m.get("brightness_start_hz"), m.get("brightness_end_hz")) {
        if e > 1.4 * s {
            words.push(format!("opens as it plays ({s:.0} → {e:.0} Hz)"));
        } else if e < s / 1.4 {
            words.push(format!("closes as it plays ({s:.0} → {e:.0} Hz)"));
        }
    }
    if let Some(&g) = m.get("glide_ms") {
        if g > 15.0 {
            words.push(format!("glides between notes (~{g:.0} ms)"));
        }
    }
    if let Some(&w) = m.get("wobble_cents") {
        if w > 6.0 {
            words.push(format!("pitch wanders (±{w:.0} cents)"));
        }
    }
    if let Some(&c) = m.get("mid_correlation") {
        words.push(
            match c {
                c if c > 0.9 => "narrow (near mono)",
                c if c > 0.6 => "slightly wide",
                c if c > 0.3 => "wide",
                _ => "very wide",
            }
            .into(),
        );
    }
    if let Some(&b) = m.get("mid_balance_db") {
        if b.abs() > 1.5 {
            words.push(format!(
                "leans {} ({:+.1} dB)",
                if b > 0.0 { "left" } else { "right" },
                b
            ));
        }
    }
    words.join(", ")
}
