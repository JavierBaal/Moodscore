//! Simple IIR filters for splitting audio into bands.

use std::f64::consts::PI;

/// A second-order section (RBJ cookbook), run in f64.
#[derive(Clone, Copy, Debug)]
struct Biquad {
    b: [f64; 3],
    a: [f64; 2],
    z: [f64; 2],
}

impl Biquad {
    fn new(kind: Kind, fc: f64, sr: f64) -> Self {
        let w = 2.0 * PI * (fc / sr).clamp(1e-5, 0.49);
        let (s, c) = w.sin_cos();
        let alpha = s / (2.0 * std::f64::consts::FRAC_1_SQRT_2);
        let a0 = 1.0 + alpha;
        let (b, a) = match kind {
            Kind::Low => (
                [(1.0 - c) / 2.0, 1.0 - c, (1.0 - c) / 2.0],
                [-2.0 * c, 1.0 - alpha],
            ),
            Kind::High => (
                [(1.0 + c) / 2.0, -(1.0 + c), (1.0 + c) / 2.0],
                [-2.0 * c, 1.0 - alpha],
            ),
        };
        Self {
            b: [b[0] / a0, b[1] / a0, b[2] / a0],
            a: [a[0] / a0, a[1] / a0],
            z: [0.0; 2],
        }
    }

    fn run(&mut self, x: f64) -> f64 {
        // Transposed direct form II.
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y
    }
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Low,
    High,
}

/// Keep `lo..hi` Hz (either end optional) with 24 dB/octave slopes.
pub fn band(x: &[f32], sr: u32, lo: Option<f32>, hi: Option<f32>) -> Vec<f32> {
    let sr = sr as f64;
    let mut stages: Vec<Biquad> = Vec::new();
    if let Some(lo) = lo.filter(|&f| f > 0.0) {
        stages.push(Biquad::new(Kind::High, lo as f64, sr));
        stages.push(Biquad::new(Kind::High, lo as f64, sr));
    }
    if let Some(hi) = hi.filter(|&f| (f as f64) < sr / 2.0) {
        stages.push(Biquad::new(Kind::Low, hi as f64, sr));
        stages.push(Biquad::new(Kind::Low, hi as f64, sr));
    }
    x.iter()
        .map(|&s| {
            let mut v = s as f64;
            for st in &mut stages {
                v = st.run(v);
            }
            v as f32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(f: f32, sr: u32, n: usize) -> Vec<f32> {
        (0..n)
            .map(|i| (std::f32::consts::TAU * f * i as f32 / sr as f32).sin())
            .collect()
    }

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt()
    }

    #[test]
    fn bands_pass_and_stop() {
        let sr = 48_000;
        let inside = band(&tone(1000.0, sr, 48_000), sr, Some(150.0), Some(3000.0));
        let below = band(&tone(40.0, sr, 48_000), sr, Some(150.0), Some(3000.0));
        let above = band(&tone(12_000.0, sr, 48_000), sr, Some(150.0), Some(3000.0));
        assert!((rms(&inside[24_000..]) / std::f32::consts::FRAC_1_SQRT_2 - 1.0).abs() < 0.05);
        assert!(rms(&below[24_000..]) < 0.01);
        assert!(rms(&above[24_000..]) < 0.01);
    }
}
