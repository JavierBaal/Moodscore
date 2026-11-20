//! Building blocks: 2× oversampling, the state-variable filter, envelopes,
//! the waveshapers.

use std::f32::consts::PI;

// ---------------------------------------------------------------- oversampling

/// Taps on each side of the halfband filter's centre that are not zero.
const HALF_TAPS: usize = 12;

/// The halfband filter's non-zero side taps, designed once (windowed sinc).
/// `h[j]` is the tap at offset `±(2j + 1)` from the centre (which is 0.5).
fn halfband_taps() -> [f32; HALF_TAPS] {
    let len = 4 * HALF_TAPS + 3;
    let centre = (len - 1) as f64 / 2.0;
    let mut h = [0.0f64; HALF_TAPS];
    for (j, t) in h.iter_mut().enumerate() {
        let off = (2 * j + 1) as f64;
        let x = off / 2.0;
        let sinc = (std::f64::consts::PI * x).sin() / (std::f64::consts::PI * x);
        // Blackman-Harris window over the full length.
        let n = centre + off;
        let w = {
            let a = 2.0 * std::f64::consts::PI * n / (len - 1) as f64;
            0.35875 - 0.48829 * a.cos() + 0.14128 * (2.0 * a).cos() - 0.01168 * (3.0 * a).cos()
        };
        *t = 0.5 * sinc * w;
    }
    // The odd-offset taps must sum to 0.5 on each phase for unity DC gain.
    let sum: f64 = h.iter().sum::<f64>() * 2.0;
    let mut out = [0.0f32; HALF_TAPS];
    for (o, t) in out.iter_mut().zip(h) {
        *o = (t * 0.5 / sum) as f32;
    }
    out
}

/// Runs a nonlinearity at twice the sample rate, with a linear-phase
/// halfband filter on the way up and on the way down.
#[derive(Clone, Debug)]
pub struct Oversampler {
    taps: [f32; HALF_TAPS],
    /// Recent input samples (upsampling), newest first.
    up: [f32; 2 * HALF_TAPS],
    /// Recent even and odd 2× samples (downsampling), newest first.
    even: [f32; 2 * HALF_TAPS],
    odd: [f32; 2 * HALF_TAPS],
}

impl Default for Oversampler {
    fn default() -> Self {
        Self {
            taps: halfband_taps(),
            up: [0.0; 2 * HALF_TAPS],
            even: [0.0; 2 * HALF_TAPS],
            odd: [0.0; 2 * HALF_TAPS],
        }
    }
}

impl Oversampler {
    pub fn clear(&mut self) {
        self.up = [0.0; 2 * HALF_TAPS];
        self.even = [0.0; 2 * HALF_TAPS];
        self.odd = [0.0; 2 * HALF_TAPS];
    }

    /// Process one sample through `f` at twice the rate.
    #[inline]
    pub fn process(&mut self, x: f32, mut f: impl FnMut(f32) -> f32) -> f32 {
        const N: usize = 2 * HALF_TAPS;
        self.up.copy_within(0..N - 1, 1);
        self.up[0] = x;
        // Interpolated sample between the inputs half a sample ago: the
        // halfband's odd taps, mirrored about the centre.
        let mut mid = 0.0;
        for j in 0..HALF_TAPS {
            mid += self.taps[j] * (self.up[HALF_TAPS - 1 - j] + self.up[HALF_TAPS + j]);
        }
        mid *= 2.0;
        // The on-grid sample is the input itself, delayed to line up.
        let on = self.up[HALF_TAPS];
        let a = f(on);
        let b = f(mid);
        // Down: the centre tap sees the on-grid stream, the odd taps the
        // in-between one.
        self.even.copy_within(0..N - 1, 1);
        self.even[0] = a;
        self.odd.copy_within(0..N - 1, 1);
        self.odd[0] = b;
        let mut y = 0.5 * self.even[HALF_TAPS - 1];
        for j in 0..HALF_TAPS {
            y += self.taps[j] * (self.odd[HALF_TAPS - 1 - j] + self.odd[HALF_TAPS + j]);
        }
        y
    }
}

// ---------------------------------------------------------------- shapers

/// Sine wavefolder: identity at `fold = 0`, folding back over itself as the
/// gain grows. Smooth (no corners), so its harmonics die away fast and
/// the 2× oversampling keeps the aliasing down.
#[inline]
pub fn fold(x: f32, amount: f32) -> f32 {
    if amount <= 0.0 {
        return x;
    }
    let g = 1.0 + 5.0 * amount;
    let folded = (std::f32::consts::FRAC_PI_2 * g * x).sin();
    let c = (4.0 * amount).min(1.0);
    x + (folded - x) * c
}

/// Soft saturation: identity at `drive = 0`, `tanh` harder as it grows,
/// with the level at full scale kept the same.
#[inline]
pub fn drive(x: f32, amount: f32) -> f32 {
    if amount <= 0.0 {
        return x;
    }
    let g = drive_gain(amount);
    (g * x).tanh() / g.tanh()
}

#[inline]
fn drive_gain(amount: f32) -> f32 {
    12.0 * amount + 1e-3
}

/// `ln(cosh(x))`, the antiderivative of `tanh`, without overflow.
#[inline]
fn log_cosh(x: f64) -> f64 {
    let a = x.abs();
    a + (-2.0 * a).exp().ln_1p() - std::f64::consts::LN_2
}

/// [`drive`] with first-order antiderivative antialiasing: the output is
/// the average of `tanh` over the segment between the last two inputs,
/// which removes most of what hard saturation would fold back.
#[derive(Clone, Copy, Debug, Default)]
pub struct Drive {
    prev: f32,
}

impl Drive {
    pub fn clear(&mut self) {
        self.prev = 0.0;
    }

    #[inline]
    pub fn process(&mut self, x: f32, amount: f32) -> f32 {
        let x0 = self.prev;
        self.prev = x;
        if amount <= 0.0 {
            // Half a sample late, like the saturated path.
            return 0.5 * (x + x0);
        }
        let g = drive_gain(amount) as f64;
        let (a, b) = (g * x0 as f64, g * x as f64);
        let y = if (b - a).abs() > 1e-6 {
            (log_cosh(b) - log_cosh(a)) / (b - a)
        } else {
            (0.5 * (a + b)).tanh()
        };
        (y / g.tanh()) as f32
    }
}

// ---------------------------------------------------------------- filter

/// Zero-delay-feedback state-variable filter (Simper's TPT form).
#[derive(Clone, Copy, Debug, Default)]
pub struct Svf {
    ic1: f32,
    ic2: f32,
}

/// Which output of the filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Low,
    Band,
    High,
}

/// Coefficients for one cutoff and damping.
#[derive(Clone, Copy, Debug)]
pub struct Coefs {
    k: f32,
    a1: f32,
    a2: f32,
    a3: f32,
}

impl Coefs {
    /// `fc` over the sample rate (clamped below Nyquist); `k` = 1/Q.
    pub fn new(fc_over_sr: f32, k: f32) -> Self {
        let g = (PI * fc_over_sr.clamp(1e-5, 0.45)).tan();
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        Self {
            k,
            a1,
            a2,
            a3: g * a2,
        }
    }
}

impl Svf {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    #[inline]
    pub fn process(&mut self, x: f32, c: &Coefs, mode: Mode) -> f32 {
        let v3 = x - self.ic2;
        let v1 = c.a1 * self.ic1 + c.a2 * v3;
        let v2 = self.ic2 + c.a2 * self.ic1 + c.a3 * v3;
        self.ic1 = 2.0 * v1 - self.ic1;
        self.ic2 = 2.0 * v2 - self.ic2;
        match mode {
            Mode::Low => v2,
            Mode::Band => v1,
            Mode::High => x - c.k * v1 - v2,
        }
    }
}

/// Damping (1/Q) for a resonance in `0..1`, from `base` (no emphasis)
/// down to nearly self-oscillating.
pub fn damping(resonance: f32, base: f32) -> f32 {
    let r = resonance.clamp(0.0, 1.0);
    base + (0.03 - base) * r.powf(0.7)
}

// ---------------------------------------------------------------- ladder

/// A four-pole transistor-ladder low-pass (the "Moog" filter): four
/// zero-delay-feedback one-pole stages in a resonant loop, with the loop's
/// input saturated (`tanh`), so it thickens and growls when driven and its
/// self-oscillation stays bounded. The feedback is solved without a unit
/// delay, so the cutoff and resonance stay where they are set at any
/// frequency. Resonance takes bass away in a real ladder; part of it is
/// given back here, so a resonant bass stays a bass.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ladder {
    s: [f32; 4],
}

/// Coefficients for one cutoff, resonance and drive.
#[derive(Clone, Copy, Debug)]
pub struct LadderCoefs {
    /// One-pole gain g / (1 + g).
    big_g: f32,
    /// 1 / (1 + g), to read a stage's state.
    inv: f32,
    /// Feedback, 0..4.06 (past 4 it self-oscillates).
    k: f32,
    /// Input gain into the saturating loop.
    gain: f32,
    /// Output make-up.
    makeup: f32,
}

impl LadderCoefs {
    /// `fc` over the sample rate, resonance and drive in `0..1`.
    pub fn new(fc_over_sr: f32, resonance: f32, drive: f32) -> Self {
        let g = (PI * fc_over_sr.clamp(1e-5, 0.45)).tan();
        let inv = 1.0 / (1.0 + g);
        // Just past 4 at full resonance: it sings on its own, held in by the tanh.
        let k = 4.06 * resonance.clamp(0.0, 1.0).powf(0.8);
        let gain = 1.0 + 6.0 * drive.clamp(0.0, 1.0);
        Self {
            big_g: g * inv,
            inv,
            k,
            gain,
            // Give back most of the bass the feedback takes (a little loss
            // is the character), and level out the drive.
            makeup: (1.0 + 0.65 * k) / gain.sqrt(),
        }
    }
}

impl Ladder {
    pub fn clear(&mut self) {
        self.s = [0.0; 4];
    }

    #[inline]
    pub fn process(&mut self, x: f32, c: &LadderCoefs) -> f32 {
        let g = c.big_g;
        // What the last stage would output for a zero input, through the chain.
        let sigma = g * g * g * self.s[0] * c.inv
            + g * g * self.s[1] * c.inv
            + g * self.s[2] * c.inv
            + self.s[3] * c.inv;
        let g4 = g * g * g * g;
        // Solve the loop linearly, then saturate what goes into it.
        let y4 = (g4 * x + sigma) / (1.0 + c.k * g4);
        let u = (c.gain * (x - c.k * y4)).tanh() / c.gain;
        let mut v = u;
        for st in &mut self.s {
            let w = (v - *st) * g;
            let y = w + *st;
            *st = y + w;
            v = y;
        }
        v * c.makeup
    }
}

// ---------------------------------------------------------------- envelope

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stage {
    Idle,
    Attack,
    Decay,
    Release,
}

/// ADSR envelope. Attack is a straight rise; decay and release fall
/// exponentially, reaching -60 dB of the way in their time. A new note
/// starts from wherever the envelope is, so retriggers never click.
#[derive(Clone, Copy, Debug)]
pub struct Env {
    pub level: f32,
    pub stage: Stage,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            level: 0.0,
            stage: Stage::Idle,
        }
    }
}

/// Seconds at the sample rate to a per-sample exponential coefficient that
/// covers 60 dB in that time.
#[inline]
fn fall_coef(seconds: f32, sr: f32) -> f32 {
    (-6.907_755 / (seconds.max(1e-4) * sr)).exp()
}

impl Env {
    pub fn gate_on(&mut self) {
        self.stage = Stage::Attack;
    }

    pub fn gate_off(&mut self) {
        if self.stage != Stage::Idle {
            self.stage = Stage::Release;
        }
    }

    pub fn is_idle(&self) -> bool {
        self.stage == Stage::Idle
    }

    /// Advance one sample. Times in seconds.
    #[inline]
    pub fn next(&mut self, a: f32, d: f32, s: f32, r: f32, sr: f32) -> f32 {
        match self.stage {
            Stage::Idle => {}
            Stage::Attack => {
                self.level += 1.0 / (a.max(1e-4) * sr);
                if self.level >= 1.0 {
                    self.level = 1.0;
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                let c = fall_coef(d, sr);
                self.level = s + (self.level - s) * c;
            }
            Stage::Release => {
                let c = fall_coef(r, sr);
                self.level *= c;
                if self.level < 1e-4 {
                    self.level = 0.0;
                    self.stage = Stage::Idle;
                }
            }
        }
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversampler_passes_low_frequencies_untouched() {
        let mut o = Oversampler::default();
        let sr = 48_000.0;
        let mut out = Vec::new();
        let mut inp = Vec::new();
        for n in 0..4800 {
            let x = (2.0 * PI * 1000.0 * n as f32 / sr).sin();
            inp.push(x);
            out.push(o.process(x, |v| v));
        }
        // Find the delay, then compare.
        let best = (0..40)
            .min_by(|&a, &b| {
                let e =
                    |d: usize| -> f32 { (1000..4000).map(|i| (out[i] - inp[i - d]).abs()).sum() };
                e(a).partial_cmp(&e(b)).unwrap()
            })
            .unwrap();
        let err = (1000..4000)
            .map(|i| (out[i] - inp[i - best]).abs())
            .fold(0.0, f32::max);
        assert!(err < 2e-3, "max error {err} at delay {best}");
    }

    #[test]
    fn shapers_are_identity_at_zero() {
        for x in [-0.9f32, -0.3, 0.0, 0.5, 1.0] {
            assert_eq!(fold(x, 0.0), x);
            assert_eq!(drive(x, 0.0), x);
            assert!(fold(x, 1.0).abs() <= 1.0);
            assert!((drive(1.0, 0.7) - 1.0).abs() < 1e-5);
        }
        // Folding stays continuous.
        let mut prev = fold(-1.0, 0.8);
        for i in 1..=2000 {
            let v = fold(-1.0 + i as f32 / 1000.0, 0.8);
            assert!((v - prev).abs() < 0.05);
            prev = v;
        }
    }

    #[test]
    fn envelope_shape() {
        let sr = 48_000.0;
        let mut e = Env::default();
        e.gate_on();
        let mut n = 0;
        while e.stage == Stage::Attack {
            e.next(0.01, 0.1, 0.5, 0.2, sr);
            n += 1;
        }
        assert!((n as f32 - 480.0).abs() < 2.0);
        for _ in 0..(0.1 * sr) as usize {
            e.next(0.01, 0.1, 0.5, 0.2, sr);
        }
        assert!((e.level - 0.5).abs() < 0.001);
        e.gate_off();
        let mut n = 0;
        while !e.is_idle() {
            e.next(0.01, 0.1, 0.5, 0.2, sr);
            n += 1;
        }
        // -60 dB in 0.2 s, idle at -80 dB from 0.5: about 0.24 s.
        assert!((n as f32 / sr - 0.243).abs() < 0.01, "{}", n as f32 / sr);
    }

    fn ladder_gain(f: f32, cutoff: f32, res: f32, amp: f32) -> f32 {
        let sr = 48_000.0;
        let c = LadderCoefs::new(cutoff / sr, res, 0.0);
        let mut l = Ladder::default();
        let mut peak = 0.0f32;
        for n in 0..19_200 {
            let y = l.process(amp * (2.0 * PI * f * n as f32 / sr).sin(), &c);
            if n > 9600 {
                peak = peak.max(y.abs());
            }
        }
        peak / amp
    }

    #[test]
    fn ladder_is_a_steep_low_pass() {
        let pass = ladder_gain(100.0, 1000.0, 0.0, 0.1);
        assert!((pass - 1.0).abs() < 0.05, "passband {pass}");
        // Four poles: about -24 dB an octave above the cutoff region.
        let two_oct = ladder_gain(4000.0, 1000.0, 0.0, 0.1);
        assert!(20.0 * two_oct.log10() < -40.0, "two octaves up: {two_oct}");
    }

    #[test]
    fn ladder_resonance_peaks_and_keeps_its_bass() {
        let flat = ladder_gain(1000.0, 1000.0, 0.0, 0.05);
        let peaky = ladder_gain(1000.0, 1000.0, 0.8, 0.05);
        assert!(peaky > 3.0 * flat, "resonance peak {flat} -> {peaky}");
        let bass0 = ladder_gain(100.0, 1000.0, 0.0, 0.05);
        let bass8 = ladder_gain(100.0, 1000.0, 0.8, 0.05);
        assert!(
            20.0 * (bass8 / bass0).log10() > -3.5,
            "bass lost: {bass0} -> {bass8}"
        );
    }

    #[test]
    fn ladder_self_oscillates_within_bounds() {
        let sr = 48_000.0;
        let c = LadderCoefs::new(440.0 / sr, 1.0, 0.0);
        let mut l = Ladder::default();
        let mut y = Vec::new();
        for n in 0..48_000 {
            let x = if n == 0 { 0.5 } else { 0.0 };
            y.push(l.process(x, &c));
        }
        let tail = &y[24_000..];
        let peak = tail.iter().fold(0.0f32, |m, v| m.max(v.abs()));
        assert!(peak > 0.05 && peak < 2.0, "self-oscillation level {peak}");
        // It rings near the cutoff.
        let crossings = tail
            .windows(2)
            .filter(|w| w[0] < 0.0 && w[1] >= 0.0)
            .count();
        let hz = crossings as f32 / 0.5;
        assert!((hz - 440.0).abs() < 60.0, "rings at {hz} Hz");
    }

    #[test]
    fn low_pass_attenuates_above_cutoff() {
        let sr = 48_000.0;
        let c = Coefs::new(1000.0 / sr, std::f32::consts::SQRT_2);
        let gain = |f: f32| {
            let mut s = Svf::default();
            let mut peak = 0.0f32;
            for n in 0..9600 {
                let y = s.process((2.0 * PI * f * n as f32 / sr).sin(), &c, Mode::Low);
                if n > 4800 {
                    peak = peak.max(y.abs());
                }
            }
            peak
        };
        assert!((gain(100.0) - 1.0).abs() < 0.02);
        assert!((gain(1000.0) - 0.707).abs() < 0.03);
        assert!(gain(8000.0) < 0.03);
    }
}
