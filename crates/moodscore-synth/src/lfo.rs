//! Low-frequency oscillators, run at control rate.

use crate::osc::Noise;

/// One LFO. Output is bipolar, `-1..1`, scaled by its delay and fade-in.
#[derive(Clone, Debug)]
pub struct Lfo {
    phase: f32,
    /// Seconds since the note started.
    elapsed: f32,
    held: f32,
    noise: Noise,
}

impl Lfo {
    pub fn new(seed: u32) -> Self {
        Self {
            phase: 0.0,
            elapsed: 0.0,
            held: 0.0,
            noise: Noise::new(seed),
        }
    }

    /// Start with a note, at `phase` (0 for a retriggered LFO; the part's
    /// running phase for a free one).
    pub fn start(&mut self, phase: f32) {
        self.phase = phase.rem_euclid(1.0);
        self.elapsed = 0.0;
        self.held = self.noise.white();
    }

    /// Advance by `dt` seconds at `hz` and return the value. `shape` is an
    /// index into [`crate::params::LFO_SHAPES`].
    pub fn advance(&mut self, dt: f32, hz: f32, shape: u8, delay: f32, fade: f32) -> f32 {
        self.phase += hz * dt;
        if self.phase >= 1.0 {
            self.phase = self.phase.fract();
            self.held = self.noise.white();
        }
        self.elapsed += dt;
        let depth = if self.elapsed < delay {
            0.0
        } else if fade > 0.0 {
            ((self.elapsed - delay) / fade).min(1.0)
        } else {
            1.0
        };
        if depth == 0.0 {
            return 0.0;
        }
        depth * shape_at(shape, self.phase, self.held)
    }
}

/// The value of an LFO shape at phase `p`.
pub fn shape_at(shape: u8, p: f32, held: f32) -> f32 {
    match shape {
        0 => (std::f32::consts::TAU * p).sin(),
        1 => 1.0 - 4.0 * ((p + 0.25).rem_euclid(1.0) - 0.5).abs(),
        2 => 2.0 * p - 1.0,
        3 => {
            if p < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        _ => held,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_are_bipolar_and_start_at_zero_crossing() {
        for s in 0..4u8 {
            let mut lo = 1.0f32;
            let mut hi = -1.0f32;
            for i in 0..1000 {
                let v = shape_at(s, i as f32 / 1000.0, 0.0);
                lo = lo.min(v);
                hi = hi.max(v);
            }
            assert!(lo <= -0.99 && hi >= 0.99, "shape {s}: {lo}..{hi}");
        }
        // Sine and triangle start from the middle, going up.
        assert!(shape_at(0, 0.0, 0.0).abs() < 1e-6);
        assert!(shape_at(1, 0.0, 0.0).abs() < 1e-6);
        assert!(shape_at(1, 0.1, 0.0) > 0.0);
    }

    #[test]
    fn delay_then_fade_in() {
        let mut l = Lfo::new(3);
        l.start(0.25); // sine at its peak
        let dt = 0.001;
        let mut t = 0.0;
        let mut first = None;
        while t < 1.5 {
            let v = l.advance(dt, 0.0, 0, 0.5, 0.5);
            if v != 0.0 && first.is_none() {
                first = Some(t);
            }
            if (t - 0.75f32).abs() < 1e-4 {
                assert!((v - 0.5).abs() < 0.01, "halfway through the fade: {v}");
            }
            t += dt;
        }
        assert!((first.unwrap() - 0.5).abs() < 0.002);
    }
}
