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
    pub const ALL: [Curve; 6] = [
        Curve::Linear,
        Curve::EaseIn,
        Curve::EaseOut,
        Curve::EaseInOut,
        Curve::Exponential,
        Curve::Logarithmic,
    ];

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

    /// The area under the curve from 0 to `u` (`u` in `0..=1`), in f64.
    ///
    /// A tempo ramp shaped by a curve covers `∫ bpm` beats, so this is what
    /// turns seconds into beats exactly, without stepping.
    pub fn integral(self, u: f64) -> f64 {
        let u = u.clamp(0.0, 1.0);
        match self {
            Curve::Linear => u * u / 2.0,
            Curve::EaseIn => u.powi(4) / 4.0,
            Curve::EaseOut => u + ((1.0 - u).powi(4) - 1.0) / 4.0,
            Curve::EaseInOut => u.powi(3) - u.powi(4) / 2.0,
            Curve::Exponential => expo_integral(u),
            Curve::Logarithmic => u - (expo_integral(1.0) - expo_integral(1.0 - u)),
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

impl std::str::FromStr for Curve {
    type Err = String;

    /// Parse a curve name. Lenient with case, spaces and hyphens
    /// (`"Ease-In"`, `"ease in"` and `"ease_in"` are the same).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let norm = s.trim().to_ascii_lowercase().replace(['-', ' '], "_");
        Curve::ALL
            .into_iter()
            .find(|c| c.name() == norm)
            .ok_or_else(|| {
                let names: Vec<&str> = Curve::ALL.iter().map(|c| c.name()).collect();
                format!("unknown curve '{s}'; use one of: {}", names.join(", "))
            })
    }
}

const EXPO_K: f32 = 5.0;

fn expo(t: f32) -> f32 {
    ((EXPO_K * t).exp() - 1.0) / (EXPO_K.exp() - 1.0)
}

/// `∫₀ᵘ expo(t) dt`.
fn expo_integral(u: f64) -> f64 {
    let k = EXPO_K as f64;
    (((k * u).exp() - 1.0) / k - u) / (k.exp() - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [Curve; 6] = Curve::ALL;

    #[test]
    fn names_parse_back_leniently() {
        for c in ALL {
            assert_eq!(c.name().parse::<Curve>(), Ok(c));
        }
        assert_eq!("Ease-In".parse::<Curve>(), Ok(Curve::EaseIn));
        assert_eq!(" ease in out ".parse::<Curve>(), Ok(Curve::EaseInOut));
        assert!("wobbly"
            .parse::<Curve>()
            .unwrap_err()
            .contains("ease_in_out"));
    }

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
    fn integrals_match_the_curves() {
        for c in ALL {
            // Midpoint rule, fine enough to check the closed forms.
            let steps = 20_000;
            let mut area = 0.0f64;
            for i in 0..steps {
                let t = (i as f64 + 0.5) / steps as f64;
                area += c.apply(t as f32) as f64 / steps as f64;
                if (i + 1) % 5_000 == 0 {
                    let u = (i + 1) as f64 / steps as f64;
                    assert!((c.integral(u) - area).abs() < 1e-5, "{c:?} at {u}");
                }
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
