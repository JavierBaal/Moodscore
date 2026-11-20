//! Spectra: magnitude spectra, spectral centroid over time, harmonic levels.

use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

/// Frame size and hop for features over time (at any sample rate).
pub const FRAME: usize = 2048;
pub const HOP: usize = 512;

fn hann(n: usize) -> Vec<f32> {
    (0..n)
        .map(|i| 0.5 - 0.5 * (std::f32::consts::TAU * i as f32 / n as f32).cos())
        .collect()
}

/// Magnitude spectrum of `x` (Hann window, zero-padded to at least `min_len`
/// and a power of two). Returns (magnitudes, Hz per bin).
pub fn magnitude(x: &[f32], sr: u32, min_len: usize) -> (Vec<f32>, f32) {
    let n = x.len().max(min_len).next_power_of_two();
    let w = hann(x.len());
    let mut buf: Vec<Complex<f32>> = (0..n)
        .map(|i| Complex::new(if i < x.len() { x[i] * w[i] } else { 0.0 }, 0.0))
        .collect();
    FftPlanner::new().plan_fft_forward(n).process(&mut buf);
    let mags = buf[..n / 2].iter().map(|c| c.norm()).collect();
    (mags, sr as f32 / n as f32)
}

/// Per frame: (spectral centroid in Hz, spectral flatness 0..1, RMS).
/// The centroid is magnitude-weighted, as most analysis tools report it.
pub fn frames(x: &[f32], sr: u32) -> Vec<(f32, f32, f32)> {
    if x.len() < FRAME {
        return Vec::new();
    }
    let w = hann(FRAME);
    let fft = FftPlanner::new().plan_fft_forward(FRAME);
    let bin = sr as f32 / FRAME as f32;
    let mut buf = vec![Complex::new(0.0f32, 0.0); FRAME];
    let mut out = Vec::new();
    let mut start = 0;
    while start + FRAME <= x.len() {
        let frame = &x[start..start + FRAME];
        for i in 0..FRAME {
            buf[i] = Complex::new(frame[i] * w[i], 0.0);
        }
        fft.process(&mut buf);
        let (mut num, mut den) = (0.0f64, 0.0f64);
        let (mut log_sum, mut lin_sum, mut count) = (0.0f64, 0.0f64, 0usize);
        for (k, c) in buf[1..FRAME / 2].iter().enumerate() {
            let m = c.norm() as f64;
            let f = (k + 1) as f64 * bin as f64;
            num += f * m;
            den += m;
            let p = m * m + 1e-20;
            log_sum += p.ln();
            lin_sum += p;
            count += 1;
        }
        let centroid = if den > 1e-9 { (num / den) as f32 } else { 0.0 };
        let flatness = ((log_sum / count as f64).exp() / (lin_sum / count as f64)) as f32;
        let rms = (frame.iter().map(|s| s * s).sum::<f32>() / FRAME as f32).sqrt();
        out.push((centroid, flatness, rms));
        start += HOP;
    }
    out
}

/// Level of harmonics 1..=count of `f0` in `x`, in dB relative to the
/// fundamental (so the first entry is 0). Each is the strongest bin within
/// ±3 % of the harmonic.
pub fn harmonics(x: &[f32], sr: u32, f0: f32, count: usize) -> Vec<f32> {
    // Pad well beyond the signal so a harmonic between bins loses < 0.1 dB.
    let (mags, bin) = magnitude(x, sr, (x.len() * 4).max(1 << 16));
    let peak = |f: f32| -> f32 {
        let lo = ((f * 0.97) / bin).floor() as usize;
        let hi = (((f * 1.03) / bin).ceil() as usize).min(mags.len().saturating_sub(1));
        if lo >= hi {
            return 0.0;
        }
        mags[lo..=hi].iter().cloned().fold(0.0, f32::max)
    };
    let fundamental = peak(f0).max(1e-12);
    (1..=count)
        .map(|k| {
            let f = f0 * k as f32;
            if f >= sr as f32 / 2.0 {
                f32::NAN
            } else {
                20.0 * (peak(f) / fundamental).max(1e-9).log10()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saw_harmonics_fall_six_db_per_octave() {
        let sr = 48_000;
        let f0 = 110.0;
        // Additive saw: harmonic k at 1/k.
        let x: Vec<f32> = (0..48_000)
            .map(|i| {
                let t = i as f64 / sr as f64;
                (1..40)
                    .map(|k| (std::f64::consts::TAU * f0 as f64 * k as f64 * t).sin() / k as f64)
                    .sum::<f64>() as f32
            })
            .collect();
        let h = harmonics(&x, sr, f0, 8);
        for (k, db) in h.iter().enumerate() {
            let expect = -20.0 * ((k + 1) as f32).log10();
            assert!((db - expect).abs() < 0.5, "harmonic {}: {db}", k + 1);
        }
    }

    #[test]
    fn centroid_of_a_tone_is_its_frequency() {
        let sr = 48_000;
        let x: Vec<f32> = (0..9600)
            .map(|i| (std::f32::consts::TAU * 1000.0 * i as f32 / sr as f32).sin())
            .collect();
        let f = frames(&x, sr);
        assert!(
            f.iter().all(|(c, _, _)| (c - 1000.0).abs() < 30.0),
            "{:?}",
            f[0]
        );
        assert!(f[0].1 < 0.01, "a tone is not noisy");
    }
}
