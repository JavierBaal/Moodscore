//! Pitch tracking (YIN, with the difference function computed by FFT), and
//! reading a pitch track as notes and glides.

use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

/// One pitch estimate per hop: Hz, or `None` where there is no clear pitch.
pub struct Track {
    pub hop_seconds: f64,
    pub hz: Vec<Option<f32>>,
}

/// YIN threshold on the cumulative mean normalised difference.
const THRESHOLD: f32 = 0.15;
/// The deepest dip still counts as a pitch below this.
const FALLBACK: f32 = 0.35;

/// Track the pitch of `x` between `fmin` and `fmax` Hz, every `hop` samples.
/// Frames quieter than `gate_db` below the loudest are left unpitched.
pub fn track(x: &[f32], sr: u32, fmin: f32, fmax: f32, hop: usize) -> Track {
    let srf = sr as f32;
    let tau_min = (srf / fmax).floor().max(2.0) as usize;
    let tau_max = (srf / fmin).ceil() as usize;
    let w = (tau_max * 2).max(1024).next_power_of_two();
    let span = w + tau_max;
    let n = (w + span).next_power_of_two();
    let mut planner = FftPlanner::<f32>::new();
    let fwd = planner.plan_fft_forward(n);
    let inv = planner.plan_fft_inverse(n);

    // Loudness gate relative to the loudest frame.
    let frame_rms = |s: usize| -> f32 {
        let e = (s + w).min(x.len());
        (x[s..e].iter().map(|v| v * v).sum::<f32>() / (e - s).max(1) as f32).sqrt()
    };
    let starts: Vec<usize> = (0..)
        .map(|i| i * hop)
        .take_while(|&s| s + span <= x.len())
        .collect();
    let loudest = starts.iter().map(|&s| frame_rms(s)).fold(0.0, f32::max);
    let gate = loudest * 10f32.powf(-40.0 / 20.0);

    let mut a = vec![Complex::new(0.0f32, 0.0); n];
    let mut b = vec![Complex::new(0.0f32, 0.0); n];
    let mut hz = Vec::with_capacity(starts.len());
    let mut d = vec![0.0f32; tau_max + 1];
    for &s in &starts {
        if frame_rms(s) < gate || loudest == 0.0 {
            hz.push(None);
            continue;
        }
        let frame = &x[s..s + span];
        for i in 0..n {
            a[i] = Complex::new(if i < w { frame[i] } else { 0.0 }, 0.0);
            b[i] = Complex::new(if i < span { frame[i] } else { 0.0 }, 0.0);
        }
        fwd.process(&mut a);
        fwd.process(&mut b);
        for i in 0..n {
            a[i] = a[i].conj() * b[i];
        }
        inv.process(&mut a);
        // r(τ) = Σ_{j<w} x_j x_{j+τ}; d(τ) = e(0) + e(τ) - 2 r(τ).
        let mut prefix = vec![0.0f64; span + 1];
        for i in 0..span {
            prefix[i + 1] = prefix[i] + (frame[i] as f64) * (frame[i] as f64);
        }
        let e0 = prefix[w];
        for (tau, dv) in d.iter_mut().enumerate() {
            let r = a[tau].re as f64 / n as f64;
            let et = prefix[tau + w] - prefix[tau];
            *dv = (e0 + et - 2.0 * r).max(0.0) as f32;
        }
        // Cumulative mean normalised difference.
        let mut running = 0.0f32;
        let mut cmnd = vec![1.0f32; tau_max + 1];
        for tau in 1..=tau_max {
            running += d[tau];
            cmnd[tau] = if running > 0.0 {
                d[tau] * tau as f32 / running
            } else {
                1.0
            };
        }
        let mut found = None;
        let mut tau = tau_min;
        while tau < tau_max {
            if cmnd[tau] < THRESHOLD {
                while tau + 1 < tau_max && cmnd[tau + 1] < cmnd[tau] {
                    tau += 1;
                }
                found = Some(tau);
                break;
            }
            tau += 1;
        }
        // None under the threshold: take the deepest dip if it is still
        // clearly periodic (as YIN does), which keeps detuned, beating
        // sounds (unison, chorus) pitched.
        let found = found.or_else(|| {
            let (best, val) = (tau_min..tau_max)
                .map(|t| (t, cmnd[t]))
                .fold((0, f32::INFINITY), |m, x| if x.1 < m.1 { x } else { m });
            (val < FALLBACK).then_some(best)
        });
        let Some(t) = found else {
            hz.push(None);
            continue;
        };
        // Parabolic interpolation around the minimum.
        let (y0, y1, y2) = (cmnd[t - 1], cmnd[t], cmnd[(t + 1).min(tau_max)]);
        let denom = y0 - 2.0 * y1 + y2;
        let shift = if denom.abs() > 1e-12 {
            (0.5 * (y0 - y2) / denom).clamp(-1.0, 1.0)
        } else {
            0.0
        };
        hz.push(Some(srf / (t as f32 + shift)));
    }
    Track {
        hop_seconds: hop as f64 / sr as f64,
        hz,
    }
}

/// MIDI note number (fractional) of a frequency.
pub fn midi(hz: f32) -> f32 {
    69.0 + 12.0 * (hz / 440.0).log2()
}

pub fn note_name(m: f32) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let r = m.round() as i32;
    format!(
        "{}{}",
        NAMES[r.rem_euclid(12) as usize],
        r.div_euclid(12) - 1
    )
}

/// A run of frames held on one note.
#[derive(Clone, Debug)]
pub struct Held {
    pub start: f64,
    pub end: f64,
    /// Nearest MIDI note.
    pub note: i32,
    /// Median deviation from it, in cents.
    pub cents: f32,
    /// Standard deviation in cents while held.
    pub wobble: f32,
}

/// Group a track into held notes (within ±35 cents of a semitone for at
/// least `min_seconds`). The gaps between them are glides or silence.
pub fn held_notes(t: &Track, min_seconds: f64) -> Vec<Held> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < t.hz.len() {
        let Some(f) = t.hz[i] else {
            i += 1;
            continue;
        };
        let note = midi(f).round() as i32;
        let mut j = i;
        let mut devs = Vec::new();
        while j < t.hz.len() {
            match t.hz[j] {
                Some(g) if (midi(g) - note as f32).abs() <= 0.35 => {
                    devs.push((midi(g) - note as f32) * 100.0);
                    j += 1;
                }
                _ => break,
            }
        }
        let dur = (j - i) as f64 * t.hop_seconds;
        if dur >= min_seconds {
            // Wobble over the middle 80 %: the edges are arrivals and departures.
            let cut = devs.len() / 10;
            let body = &devs[cut..devs.len() - cut];
            let mean = body.iter().sum::<f32>() / body.len() as f32;
            let var = body.iter().map(|d| (d - mean).powi(2)).sum::<f32>() / body.len() as f32;
            devs.sort_by(|a, b| a.total_cmp(b));
            let median = devs[devs.len() / 2];
            out.push(Held {
                start: i as f64 * t.hop_seconds,
                end: j as f64 * t.hop_seconds,
                note,
                cents: median,
                wobble: var.sqrt(),
            });
        }
        i = j.max(i + 1);
    }
    out
}

/// Time between consecutive held notes on different pitches where the pitch
/// slid between them: glides, in seconds. A gap counts when at least half of
/// it is pitched and every pitch heard in it lies between the two notes (in
/// a mix, some frames of a slide are lost to other parts).
pub fn glides(t: &Track, held: &[Held]) -> Vec<f64> {
    held.windows(2)
        .filter(|w| w[0].note != w[1].note)
        .filter_map(|w| {
            let (a, b) = (
                (w[0].end / t.hop_seconds).round() as usize,
                ((w[1].start / t.hop_seconds).round() as usize).min(t.hz.len()),
            );
            let gap = b.saturating_sub(a);
            if gap == 0 || gap as f64 * t.hop_seconds > 0.6 {
                return None;
            }
            let (lo, hi) = (
                w[0].note.min(w[1].note) as f32 - 0.5,
                w[0].note.max(w[1].note) as f32 + 0.5,
            );
            let heard: Vec<f32> = (a..b).filter_map(|k| t.hz[k]).map(midi).collect();
            let between = heard.iter().all(|m| (lo..=hi).contains(m));
            (heard.len() * 2 >= gap && between).then_some(gap as f64 * t.hop_seconds)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sweep(sr: u32, parts: &[(f32, f32, f32)]) -> Vec<f32> {
        // (seconds, from Hz, to Hz), phase continuous, with a few harmonics.
        let mut out = Vec::new();
        let mut ph = 0.0f32;
        for &(secs, a, b) in parts {
            let n = (secs * sr as f32) as usize;
            for i in 0..n {
                let f = a + (b - a) * i as f32 / n as f32;
                ph = (ph + f / sr as f32).fract();
                let p = std::f32::consts::TAU * ph;
                out.push(p.sin() + 0.5 * (2.0 * p).sin() + 0.3 * (3.0 * p).sin());
            }
        }
        out
    }

    #[test]
    fn tracks_notes_and_glides() {
        let sr = 48_000;
        // A3 for 0.5 s, glide to E4 over 0.1 s, hold 0.5 s.
        let x = sweep(
            sr,
            &[
                (0.5, 220.0, 220.0),
                (0.1, 220.0, 329.63),
                (0.5, 329.63, 329.63),
            ],
        );
        let t = track(&x, sr, 80.0, 1000.0, 128);
        let held = held_notes(&t, 0.05);
        let names: Vec<String> = held.iter().map(|h| note_name(h.note as f32)).collect();
        assert_eq!(names, ["A3", "E4"], "{held:?}");
        assert!(
            held.iter().all(|h| h.cents.abs() < 3.0 && h.wobble < 3.0),
            "{held:?}"
        );
        let g = glides(&t, &held);
        assert_eq!(g.len(), 1);
        // Within ±35 cents counts as arrived: a bit under the full 0.1 s.
        assert!(g[0] > 0.05 && g[0] < 0.12, "glide {}", g[0]);
    }

    #[test]
    fn low_notes_too() {
        let sr = 44_100;
        let x = sweep(sr, &[(1.0, 58.27, 58.27)]); // A#1
        let t = track(&x, sr, 30.0, 200.0, 512);
        let held = held_notes(&t, 0.2);
        assert_eq!(held.len(), 1);
        assert_eq!(note_name(held[0].note as f32), "A#1");
    }
}
