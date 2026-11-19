//! Spectrum helpers shared by the synth tests.
#![allow(dead_code)]

/// Magnitude spectrum (dB, normalised to the strongest bin) of `x`, with a
/// Blackman-Harris window. Naive DFT: fine for test sizes.
pub fn spectrum_db(x: &[f32]) -> Vec<f32> {
    let n = x.len();
    let w: Vec<f64> = (0..n)
        .map(|i| {
            let a = 2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64;
            0.35875 - 0.48829 * a.cos() + 0.14128 * (2.0 * a).cos() - 0.01168 * (3.0 * a).cos()
        })
        .collect();
    let mut mags = vec![0.0f64; n / 2];
    // Real DFT with a rotating phasor per bin.
    for (k, m) in mags.iter_mut().enumerate() {
        let step = -2.0 * std::f64::consts::PI * k as f64 / n as f64;
        let (sr, si) = (step.cos(), step.sin());
        let (mut c, mut s) = (1.0f64, 0.0f64);
        let (mut re, mut im) = (0.0, 0.0);
        for i in 0..n {
            let v = x[i] as f64 * w[i];
            re += v * c;
            im += v * s;
            let nc = c * sr - s * si;
            s = c * si + s * sr;
            c = nc;
        }
        *m = (re * re + im * im).sqrt();
    }
    let peak = mags.iter().cloned().fold(1e-30, f64::max);
    mags.iter()
        .map(|m| (20.0 * (m / peak).max(1e-12).log10()) as f32)
        .collect()
}

/// Strongest bin that is not within `guard` bins of a harmonic of `f0`
/// (or of DC), in dB, below `limit` Hz.
pub fn worst_alias_db(spec: &[f32], f0: f32, sr: f32, guard: usize, limit: f32) -> f32 {
    let n = spec.len() * 2;
    let bin_hz = sr / n as f32;
    let mut worst = -200.0f32;
    for (k, &db) in spec.iter().enumerate() {
        let f = k as f32 * bin_hz;
        if f > limit {
            break;
        }
        let h = (f / f0).round();
        let near = (f - h * f0).abs() / bin_hz <= guard as f32;
        if !near {
            worst = worst.max(db);
        }
    }
    worst
}
