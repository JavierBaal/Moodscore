//! A three-band level meter: how much energy a signal has below 200 Hz,
//! from 200 Hz to 4 kHz, and above, summed over the samples it is fed.
//!
//! The bands are the ones the drones make room in for the notes, so a
//! part's level can be set against the drones' band by band (is it heard,
//! or buried?). Cheap enough for the audio thread: eight biquads a sample,
//! no allocation.

use std::f32::consts::PI;

/// The band edges, in Hz.
pub const BAND_EDGES: [f32; 2] = [200.0, 4000.0];

/// The bands' names, low to high.
pub const BANDS: [&str; 3] = ["low", "mid", "high"];

/// A second-order Butterworth filter (RBJ cookbook, direct form I).
#[derive(Clone, Copy, Debug)]
struct Biquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl Biquad {
    fn new(sample_rate: f32, cutoff: f32, high: bool) -> Self {
        let w = 2.0 * PI * cutoff / sample_rate;
        let alpha = w.sin() / std::f32::consts::SQRT_2;
        let cos = w.cos();
        let a0 = 1.0 + alpha;
        let (b0, b1) = if high {
            ((1.0 + cos) / 2.0, -(1.0 + cos))
        } else {
            ((1.0 - cos) / 2.0, 1.0 - cos)
        };
        Biquad {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b0 / a0,
            a1: -2.0 * cos / a0,
            a2: (1.0 - alpha) / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        // Flush denormals as the signal dies away.
        self.y1 = if y.abs() < 1e-20 { 0.0 } else { y };
        y
    }
}

/// A Linkwitz-Riley (fourth-order) low- or high-pass: two Butterworths in
/// a row, so a band split leaks little into its neighbours.
#[derive(Clone, Copy, Debug)]
struct Lr4([Biquad; 2]);

impl Lr4 {
    fn new(sample_rate: f32, cutoff: f32, high: bool) -> Self {
        let b = Biquad::new(sample_rate, cutoff, high);
        Lr4([b, b])
    }

    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.0[0].run(x);
        self.0[1].run(y)
    }
}

/// Energy in the three bands of what it is fed since it was last taken.
#[derive(Clone, Copy, Debug)]
pub struct BandMeter {
    low: Lr4,
    mid: [Lr4; 2],
    high: Lr4,
    energy: [f32; 3],
}

impl BandMeter {
    pub fn new(sample_rate: f32) -> Self {
        let [a, b] = BAND_EDGES;
        BandMeter {
            low: Lr4::new(sample_rate, a, false),
            mid: [
                Lr4::new(sample_rate, a, true),
                Lr4::new(sample_rate, b, false),
            ],
            high: Lr4::new(sample_rate, b, true),
            energy: [0.0; 3],
        }
    }

    /// Add one sample.
    #[inline]
    pub fn feed(&mut self, x: f32) {
        let low = self.low.run(x);
        let above_low = self.mid[0].run(x);
        let mid = self.mid[1].run(above_low);
        let high = self.high.run(x);
        for (e, b) in self.energy.iter_mut().zip([low, mid, high]) {
            *e += b * b;
        }
    }

    /// The energy fed since the last call (sum of squares per band), and
    /// start again.
    pub fn take(&mut self) -> [f32; 3] {
        std::mem::take(&mut self.energy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(hz: f32) -> [f32; 3] {
        let sr = 48_000.0;
        let mut m = BandMeter::new(sr);
        for i in 0..48_000 {
            m.feed((2.0 * PI * hz * i as f32 / sr).sin());
        }
        m.take()
    }

    #[test]
    fn a_tone_lands_in_its_band() {
        for (hz, band) in [(60.0, 0), (1000.0, 1), (9000.0, 2)] {
            let e = tone(hz);
            let total: f32 = e.iter().sum();
            assert!(e[band] / total > 0.9, "{hz} Hz: {e:?}");
            // About the tone's own energy (half a second's worth of 1.0^2).
            assert!((total / 24_000.0 - 1.0).abs() < 0.2, "{hz} Hz: {total}");
        }
    }
}
