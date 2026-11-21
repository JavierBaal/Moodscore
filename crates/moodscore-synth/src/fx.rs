//! A part's effects: saturation, chorus and a tempo-synced echo.
//!
//! Buffers are allocated when the part is created; processing never
//! allocates. Width comes from modulating or timing both sides
//! symmetrically, never from delaying the dry sound on one side.

use crate::dsp::Drive;

/// A Butterworth second-order section (RBJ), for the ensemble's crossover.
#[derive(Clone, Copy, Debug, Default)]
struct Biquad {
    b: [f32; 3],
    a: [f32; 2],
    z: [f32; 2],
}

impl Biquad {
    fn new(high: bool, fc: f32, sr: f32) -> Self {
        let w = std::f32::consts::TAU * fc / sr;
        let (sn, cs) = w.sin_cos();
        let alpha = sn / (2.0 * std::f32::consts::FRAC_1_SQRT_2);
        let a0 = 1.0 + alpha;
        let b = if high {
            [(1.0 + cs) / 2.0, -(1.0 + cs), (1.0 + cs) / 2.0]
        } else {
            [(1.0 - cs) / 2.0, 1.0 - cs, (1.0 - cs) / 2.0]
        };
        Self {
            b: [b[0] / a0, b[1] / a0, b[2] / a0],
            a: [-2.0 * cs / a0, (1.0 - alpha) / a0],
            z: [0.0; 2],
        }
    }

    #[inline]
    fn run(&mut self, x: f32) -> f32 {
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y
    }
}

/// A Linkwitz-Riley crossover (24 dB/octave): low and high bands that add
/// back up to the input's level.
#[derive(Clone, Copy, Debug)]
struct Crossover {
    lp: [Biquad; 2],
    hp: [Biquad; 2],
}

impl Crossover {
    fn new(fc: f32, sr: f32) -> Self {
        Self {
            lp: [Biquad::new(false, fc, sr); 2],
            hp: [Biquad::new(true, fc, sr); 2],
        }
    }

    fn clear(&mut self) {
        for b in self.lp.iter_mut().chain(self.hp.iter_mut()) {
            b.z = [0.0; 2];
        }
    }

    #[inline]
    fn split(&mut self, x: f32) -> (f32, f32) {
        let l1 = self.lp[0].run(x);
        let lo = self.lp[1].run(l1);
        let h1 = self.hp[0].run(x);
        // LR4 (two Butterworth sections each): the bands add up in phase.
        let hi = self.hp[1].run(h1);
        (lo, hi)
    }
}

/// A delay line with fractional (linear) reads.
#[derive(Clone, Debug)]
struct Line {
    buf: Vec<f32>,
    pos: usize,
}

impl Line {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(4)],
            pos: 0,
        }
    }

    fn push(&mut self, x: f32) {
        self.pos = (self.pos + 1) % self.buf.len();
        self.buf[self.pos] = x;
    }

    /// The sample `delay` samples ago (`1 ..= len - 2`).
    fn read(&self, delay: f32) -> f32 {
        let n = self.buf.len();
        let d = delay.clamp(1.0, (n - 2) as f32);
        let i = d.floor();
        let f = d - i;
        let a = self.buf[(self.pos + n - i as usize) % n];
        let b = self.buf[(self.pos + n - i as usize - 1) % n];
        a + (b - a) * f
    }

    fn clear(&mut self) {
        self.buf.iter_mut().for_each(|s| *s = 0.0);
    }
}

/// Settings read once per block.
#[derive(Copy, Clone, Debug, Default)]
pub struct FxParams {
    pub distortion: f32,
    pub ensemble: f32,
    pub chorus_mix: f32,
    pub chorus_rate: f32,
    pub chorus_depth: f32,
    pub delay_mix: f32,
    /// Echo time in seconds.
    pub delay_time: f32,
    pub delay_feedback: f32,
    pub delay_width: f32,
    pub delay_tone: f32,
}

pub struct Fx {
    sr: f32,
    drive: [Drive; 2],
    ensemble: Line,
    ens_slow: f32,
    ens_fast: f32,
    /// Crossover per side: below it, the ensemble leaves the sound alone.
    ens_split: [Crossover; 2],
    chorus: [Line; 2],
    chorus_phase: f32,
    echo: [Line; 2],
    /// Smoothed echo times (samples), so tempo changes glide.
    echo_time: [f32; 2],
    echo_lp: [f32; 2],
}

/// Below this the ensemble leaves the sound alone (Hz).
const ENSEMBLE_CROSSOVER: f32 = 140.0;

/// Longest echo, in seconds.
pub const MAX_DELAY: f32 = 4.0;

impl Fx {
    pub fn new(sample_rate: u32) -> Self {
        let sr = sample_rate as f32;
        let chorus_len = (0.05 * sr) as usize;
        let echo_len = (MAX_DELAY * 1.5 * sr) as usize + 4;
        Self {
            sr,
            drive: Default::default(),
            ensemble: Line::new((0.03 * sr) as usize),
            ens_slow: 0.0,
            ens_fast: 0.0,
            ens_split: [Crossover::new(ENSEMBLE_CROSSOVER, sr); 2],
            chorus: [Line::new(chorus_len), Line::new(chorus_len)],
            chorus_phase: 0.0,
            echo: [Line::new(echo_len), Line::new(echo_len)],
            echo_time: [0.0; 2],
            echo_lp: [0.0; 2],
        }
    }

    pub fn clear(&mut self) {
        for l in self.chorus.iter_mut().chain(self.echo.iter_mut()) {
            l.clear();
        }
        self.ensemble.clear();
        for x in &mut self.ens_split {
            x.clear();
        }
        self.echo_lp = [0.0; 2];
    }

    /// Let `n` frames of silence go by without computing them: the sweeps
    /// move on and the echo time glides, as if silence had been processed
    /// (the lines hold silence already).
    pub fn idle(&mut self, p: &FxParams, n: usize) {
        let (sr, n) = (self.sr, n as f32);
        self.ens_slow = (self.ens_slow + n * 0.62 / sr).fract();
        self.ens_fast = (self.ens_fast + n * 5.7 / sr).fract();
        self.chorus_phase = (self.chorus_phase + n * p.chorus_rate / sr).fract();
        if self.echo_time[0] != 0.0 {
            let target = [
                (p.delay_time * sr).clamp(2.0, MAX_DELAY * sr),
                (p.delay_time * (1.0 + 0.5 * p.delay_width) * sr).clamp(2.0, MAX_DELAY * 1.5 * sr),
            ];
            let keep = (-n / (0.1 * sr)).exp();
            for (t, goal) in self.echo_time.iter_mut().zip(target) {
                *t = goal + (*t - goal) * keep;
            }
        }
    }

    /// Process in place.
    pub fn process(&mut self, p: &FxParams, left: &mut [f32], right: &mut [f32]) {
        let sr = self.sr;
        let base = 0.012 * sr;
        let depth = 0.004 * sr * p.chorus_depth;
        let chorus_inc = p.chorus_rate / sr;
        let target = [
            (p.delay_time * sr).clamp(2.0, MAX_DELAY * sr),
            (p.delay_time * (1.0 + 0.5 * p.delay_width) * sr).clamp(2.0, MAX_DELAY * 1.5 * sr),
        ];
        if self.echo_time[0] == 0.0 {
            self.echo_time = target;
        }
        // 100 ms glide of the echo time.
        let glide = 1.0 - (-1.0 / (0.1 * sr)).exp();
        let tone = 1.0 - (-std::f32::consts::TAU * p.delay_tone / sr).exp();
        let chorus_norm = 1.0 / (1.0 + 0.5 * p.chorus_mix);
        // Ensemble: three taps per side on one line, swept by a slow and a
        // fast sine (the string-machine recipe), the right side's taps a
        // sixth of a turn from the left's.
        let (ens_base, ens_slow_depth, ens_fast_depth) = (0.008 * sr, 0.0025 * sr, 0.0003 * sr);
        let (slow_inc, fast_inc) = (0.62 / sr, 5.7 / sr);
        let ens = p.ensemble.clamp(0.0, 1.0);
        for i in 0..left.len() {
            let mut x = [left[i], right[i]];
            if p.distortion > 0.0 {
                for (c, s) in x.iter_mut().enumerate() {
                    *s = self.drive[c].process(*s, p.distortion);
                }
            }
            // Split: the lows pass untouched (mono, tight), the rest goes
            // through the ensemble.
            let (mut lows, mut highs) = ([0.0f32; 2], [0.0f32; 2]);
            for c in 0..2 {
                (lows[c], highs[c]) = self.ens_split[c].split(x[c]);
            }
            self.ensemble.push(0.5 * (highs[0] + highs[1]));
            self.ens_slow = (self.ens_slow + slow_inc).fract();
            self.ens_fast = (self.ens_fast + fast_inc).fract();
            if ens > 0.0 {
                let mut wet = [0.0f32; 2];
                for (side, w) in wet.iter_mut().enumerate() {
                    for tap in 0..3 {
                        let turn = tap as f32 / 3.0 + side as f32 / 6.0;
                        let slow = (std::f32::consts::TAU * (self.ens_slow + turn)).sin();
                        let fast = (std::f32::consts::TAU * (self.ens_fast + turn)).sin();
                        *w += self
                            .ensemble
                            .read(ens_base + ens_slow_depth * slow + ens_fast_depth * fast);
                    }
                    // Three loosely related copies add up like noise: by √3.
                    *w *= 0.577;
                }
                for c in 0..2 {
                    x[c] = lows[c] + highs[c] * (1.0 - 0.5 * ens) + wet[c] * 0.9 * ens;
                }
            }
            // Chorus: each side through its own delay line, swept in
            // quadrature, mixed back in.
            self.chorus_phase = (self.chorus_phase + chorus_inc).fract();
            let ph = std::f32::consts::TAU * self.chorus_phase;
            let sweep = [ph.sin(), ph.cos()];
            for c in 0..2 {
                self.chorus[c].push(x[c]);
                if p.chorus_mix > 0.0 {
                    let d = self.chorus[c].read(base + depth * sweep[c]);
                    x[c] = (x[c] + p.chorus_mix * d) * chorus_norm;
                }
            }
            // Echo: two lines with their own feedback, darker each repeat.
            for c in 0..2 {
                self.echo_time[c] += (target[c] - self.echo_time[c]) * glide;
                let e = self.echo[c].read(self.echo_time[c]);
                self.echo_lp[c] += (e - self.echo_lp[c]) * tone;
                self.echo[c].push(x[c] + self.echo_lp[c] * p.delay_feedback);
                x[c] += e * p.delay_mix;
            }
            left[i] = x[0];
            right[i] = x[1];
        }
    }
}
