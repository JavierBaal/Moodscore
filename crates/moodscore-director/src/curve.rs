//! Gesture curves: how a value travels from start to end over time.

use serde::{Deserialize, Serialize};

/// The shape of a gesture over its duration.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Curve {
    /// Constant speed.
    Linear,
    /// Starts slowly, speeds up.
    EaseIn,
    /// Starts quickly, settles gently.
    EaseOut,
    /// Slow start, slow finish. The natural default for musical motion.
    #[default]
    EaseInOut,
    /// Barely moves at first, then rushes to the end. For arrivals and hits.
    Exponential,
    /// Moves most at the start, then creeps toward the end.
    Logarithmic,
}

impl Curve {
    /// Map progress `t` in `0..=1` to eased progress in `0..=1`.
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Curve::Linear => t,
            Curve::EaseIn => t * t * t,
            Curve::EaseOut => 1.0 - (1.0 - t).powi(3),
            Curve::EaseInOut => t * t * (3.0 - 2.0 * t),
            Curve::Exponential => expo(t),
            Curve::Logarithmic => 1.0 - expo(1.0 - t),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Curve::Linear => "linear",
            Curve::EaseIn => "ease_in",
            Curve::EaseOut => "ease_out",
            Curve::EaseInOut => "ease_in_out",
            Curve::Exponential => "exponential",
            Curve::Logarithmic => "logarithmic",
        }
    }
}

fn expo(t: f32) -> f32 {
    const K: f32 = 5.0;
    ((K * t).exp() - 1.0) / (K.exp() - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Curve; 6] = [
        Curve::Linear,
        Curve::EaseIn,
        Curve::EaseOut,
        Curve::EaseInOut,
        Curve::Exponential,
        Curve::Logarithmic,
    ];

    #[test]
    fn every_curve_starts_at_0_ends_at_1_and_is_monotonic() {
        for c in ALL {
            assert!(c.apply(0.0).abs() < 1e-6, "{c:?} start");
            assert!((c.apply(1.0) - 1.0).abs() < 1e-6, "{c:?} end");
            let mut prev = 0.0;
            for i in 1..=100 {
                let v = c.apply(i as f32 / 100.0);
                assert!(v >= prev - 1e-6, "{c:?} not monotonic");
                prev = v;
            }
        }
    }

    #[test]
    fn shapes_lean_the_right_way() {
        assert!(Curve::EaseIn.apply(0.5) < 0.5);
        assert!(Curve::EaseOut.apply(0.5) > 0.5);
        assert!(Curve::Exponential.apply(0.5) < Curve::EaseIn.apply(0.5));
        assert!(Curve::Logarithmic.apply(0.5) > 0.5);
    }
}
