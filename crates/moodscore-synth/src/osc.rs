//! Band-limited morphing oscillator.
//!
//! The waveform morphs continuously from sine (0) through triangle (1) and
//! saw (2) to square (3, with pulse width). Every corner and jump of the
//! mixed waveform is smoothed with a two-sample polynomial correction
//! (PolyBLEP for jumps, PolyBLAMP for corners), placed at the exact
//! fraction of a sample where it happens. Hard sync restarts the cycle from
//! a hidden master running at the note's frequency, and the jump that
//! causes is corrected the same way.
//!
//! The corrections reach one sample back, so the oscillator runs one sample
//! late: it holds the previous sample until the next call.

use std::f32::consts::TAU;

/// Mix weights of the four basic shapes for a morph position in `0..=3`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Shape {
    pub sine: f32,
    pub tri: f32,
    pub saw: f32,
    pub square: f32,
    /// Pulse width of the square, `0.05..0.95`.
    pub pw: f32,
}

impl Shape {
    pub fn new(morph: f32, pw: f32) -> Self {
        let m = morph.clamp(0.0, 3.0);
        let k = (m.floor() as usize).min(2);
        let f = m - k as f32;
        let mut w = [0.0f32; 4];
        w[k] = 1.0 - f;
        w[k + 1] = f;
        Self {
            sine: w[0],
            tri: w[1],
            saw: w[2],
            square: w[3],
            pw: pw.clamp(0.05, 0.95),
        }
    }

    /// The naive (not band-limited) waveform at phase `p` in `0..1`.
    pub fn value(&self, p: f32) -> f32 {
        let mut v = 0.0;
        if self.sine != 0.0 {
            v += self.sine * (TAU * p).sin();
        }
        if self.tri != 0.0 {
            v += self.tri * (1.0 - 4.0 * (p - 0.5).abs());
        }
        if self.saw != 0.0 {
            v += self.saw * (2.0 * p - 1.0);
        }
        if self.square != 0.0 {
            v += self.square * if p < self.pw { 1.0 } else { -1.0 };
        }
        v
    }

    /// Slope of the naive waveform at `p`, per cycle.
    fn slope(&self, p: f32) -> f32 {
        let mut s = 0.0;
        if self.sine != 0.0 {
            s += self.sine * TAU * (TAU * p).cos();
        }
        if self.tri != 0.0 {
            s += self.tri * if p < 0.5 { 4.0 } else { -4.0 };
        }
        s + self.saw * 2.0
    }
}

/// One oscillator.
#[derive(Clone, Debug, Default)]
pub struct Osc {
    phase: f32,
    /// Hidden master for hard sync, at the note's frequency.
    master: f32,
    /// The sample being finished (output on the next call).
    held: f32,
    /// Corrections for the sample being computed now.
    cur: f32,
}

impl Osc {
    /// Restart at phase `p` (both the oscillator and its sync master).
    pub fn reset(&mut self, p: f32) {
        self.phase = p.rem_euclid(1.0);
        self.master = self.phase;
        self.held = 0.0;
        self.cur = 0.0;
    }

    /// Add a jump of `h` and a slope change of `ds` (per sample), `d` of a
    /// sample ago.
    fn event(&mut self, d: f32, h: f32, ds: f32) {
        let d = d.clamp(0.0, 1.0);
        let e = 1.0 - d;
        // Two-sample BLEP / BLAMP residuals: the previous sample sits at
        // d - 1 from the event, the current one at d.
        self.held += h * 0.5 * d * d + ds * d * d * d / 6.0;
        self.cur += -h * 0.5 * e * e + ds * e * e * e / 6.0;
    }

    /// Corners and jumps strictly after `a`, up to and including `b`, in one
    /// cycle (`0 <= a < b <= 1`). `after` is the time from `b` to now, in samples.
    fn edges(&mut self, s: &Shape, a: f32, b: f32, inc: f32, after: f32) {
        if s.tri != 0.0 && a < 0.5 && 0.5 <= b {
            self.event((b - 0.5) / inc + after, 0.0, -8.0 * inc * s.tri);
        }
        if s.square != 0.0 && a < s.pw && s.pw <= b {
            self.event((b - s.pw) / inc + after, -2.0 * s.square, 0.0);
        }
    }

    /// Move the phase from `a` to `b` (`b - a < 1`), wrapping once at most.
    /// Returns the new phase.
    fn travel(&mut self, s: &Shape, a: f32, b: f32, inc: f32, after: f32) -> f32 {
        if b < 1.0 {
            self.edges(s, a, b, inc, after);
            return b;
        }
        let b = b - 1.0;
        let d = b / inc + after;
        self.edges(s, a, 1.0, inc, d);
        // Wrap: the saw falls 2, the square rises 2, the triangle turns up.
        self.event(d, -2.0 * s.saw + 2.0 * s.square, 8.0 * inc * s.tri);
        self.edges(s, 0.0, b, inc, after);
        b
    }

    /// Next sample. `inc` is the frequency over the sample rate (`< 0.5`);
    /// `sync` the hard-sync ratio (`1` = off): the oscillator then runs at
    /// `inc` and restarts every `sync` of its cycles.
    pub fn next(&mut self, s: &Shape, inc: f32, sync: f32) -> f32 {
        let inc = inc.clamp(1e-7, 0.49);
        let p0 = self.phase;
        if sync > 1.0005 {
            let minc = inc / sync;
            let m1 = self.master + minc;
            if m1 >= 1.0 {
                // The master restarts `dm` samples ago; we reach `mid` then.
                let dm = ((m1 - 1.0) / minc).min(1.0);
                self.master = m1 - 1.0;
                let mid = self.travel(s, p0, p0 + inc * (1.0 - dm), inc, dm);
                let to = (inc * dm).min(0.999);
                let h = s.value(0.0) - s.value(mid);
                let ds = (s.slope(0.0) - s.slope(mid)) * inc;
                self.event(dm, h, ds);
                self.phase = self.travel(s, 0.0, to.max(1e-9), inc, 0.0);
                return self.finish(s);
            }
            self.master = m1;
        }
        self.phase = self.travel(s, p0, p0 + inc, inc, 0.0);
        self.finish(s)
    }

    fn finish(&mut self, s: &Shape) -> f32 {
        let out = self.held;
        self.held = s.value(self.phase) + self.cur;
        self.cur = 0.0;
        out
    }
}

/// A fast, deterministic white-noise source.
#[derive(Clone, Debug)]
pub struct Noise(u32);

impl Noise {
    pub fn new(seed: u32) -> Self {
        Self(seed.max(1))
    }

    /// Uniform in `-1..1`.
    pub fn white(&mut self) -> f32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        (x as f32 / u32::MAX as f32) * 2.0 - 1.0
    }

    /// Uniform in `0..1`.
    pub fn unit(&mut self) -> f32 {
        0.5 * (self.white() + 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_mix_two_neighbours() {
        let s = Shape::new(2.25, 0.5);
        assert!((s.saw - 0.75).abs() < 1e-6 && (s.square - 0.25).abs() < 1e-6);
        assert_eq!(s.sine + s.tri, 0.0);
        let s = Shape::new(3.0, 0.5);
        assert_eq!(s.square, 1.0);
    }

    #[test]
    fn output_stays_bounded_and_dc_free() {
        for morph in [0.0, 0.5, 1.0, 1.7, 2.0, 2.5, 3.0] {
            let s = Shape::new(morph, 0.5);
            let mut o = Osc::default();
            let n = 48_000;
            let mut sum = 0.0f64;
            for _ in 0..n {
                let v = o.next(&s, 441.0 / 48_000.0, 1.0);
                assert!(v.abs() < 1.2, "morph {morph}: {v}");
                sum += v as f64;
            }
            assert!(
                (sum / n as f64).abs() < 0.01,
                "morph {morph} DC {}",
                sum / n as f64
            );
        }
    }
}
