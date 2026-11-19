//! A part's effects: saturation, chorus and a tempo-synced echo.
//!
//! Buffers are allocated when the part is created; processing never
//! allocates. Width comes from modulating or timing both sides
//! symmetrically, never from delaying the dry sound on one side.

use crate::dsp::Drive;

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
    chorus: [Line; 2],
    chorus_phase: f32,
    echo: [Line; 2],
    /// Smoothed echo times (samples), so tempo changes glide.
    echo_time: [f32; 2],
    echo_lp: [f32; 2],
}

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
        self.echo_lp = [0.0; 2];
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
        for i in 0..left.len() {
            let mut x = [left[i], right[i]];
            if p.distortion > 0.0 {
                for (c, s) in x.iter_mut().enumerate() {
                    *s = self.drive[c].process(*s, p.distortion);
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
