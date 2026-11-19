//! One sounding note: two unison oscillator banks, sub, noise, the
//! oversampled shaper, the filter and three envelopes.

use crate::dsp::{self, Coefs, Drive, Env, Mode, Oversampler, Svf};
use crate::osc::{Noise, Osc, Shape};
use crate::params::{P, PARAMS};

/// Most unison copies per oscillator.
pub const MAX_UNISON: usize = 7;

/// Samples between control updates (unison spread, tuning, shapes).
pub const CONTROL_BLOCK: usize = 16;

/// What the voice reads once per control block, from the (modulated) patch.
#[derive(Clone, Copy, Debug, Default)]
struct Bank {
    level: f32,
    shape: Shape,
    /// Frequency ratio of each copy to the note.
    ratio: [f32; MAX_UNISON],
    /// Left and right gain of each copy.
    gain: [(f32, f32); MAX_UNISON],
    sync: f32,
}

/// One note being played.
#[derive(Clone, Debug)]
pub struct Voice {
    sr: f32,
    osc: [[Osc; MAX_UNISON]; 2],
    unison: [usize; 2],
    sub_phase: f32,
    noise: Noise,
    shaper: [Oversampler; 2],
    drive: [Drive; 2],
    filter: [[Svf; 2]; 2],
    pub(crate) amp: Env,
    pub(crate) fenv: Env,
    pub(crate) menv: Env,
    /// Pitch in MIDI notes, gliding toward `target`.
    pub(crate) pitch: f32,
    pub(crate) target: f32,
    pub(crate) velocity: f32,
    /// Discrete settings, fixed when the note starts.
    filter_mode: Mode,
    steep: bool,
    sub_octave: f32,
    /// The note held, and whether its key is still down.
    pub(crate) note: Option<u8>,
    pub(crate) held: bool,
    /// The previous FM modulator sample (oscillator 2, mono).
    fm_prev: f32,
    banks: [Bank; 2],
    /// Samples left in the current control block.
    ctl_left: usize,
    /// Patch values with modulation applied, refreshed each control block.
    pub(crate) live: Vec<f32>,
}

impl Voice {
    pub fn new(sample_rate: u32, seed: u32) -> Self {
        Self {
            sr: sample_rate as f32,
            osc: Default::default(),
            unison: [1, 1],
            sub_phase: 0.0,
            noise: Noise::new(seed.wrapping_mul(2_654_435_761).max(1)),
            shaper: Default::default(),
            drive: Default::default(),
            filter: Default::default(),
            amp: Env::default(),
            fenv: Env::default(),
            menv: Env::default(),
            pitch: 60.0,
            target: 60.0,
            velocity: 1.0,
            filter_mode: Mode::Low,
            steep: false,
            sub_octave: -1.0,
            note: None,
            held: false,
            fm_prev: 0.0,
            banks: Default::default(),
            ctl_left: 0,
            live: PARAMS.iter().map(|p| p.init).collect(),
        }
    }

    pub fn is_active(&self) -> bool {
        !self.amp.is_idle()
    }

    /// Start a note. If the voice is still sounding (a retrigger, or a
    /// stolen voice), the envelopes rise from where they are: no click.
    pub fn start(&mut self, patch: &[f32], note: u8, velocity: f32, glide_from: Option<f32>) {
        let was_silent = !self.is_active();
        self.note = Some(note);
        self.held = true;
        self.velocity = velocity.clamp(0.0, 1.0);
        self.target = note as f32;
        self.pitch = glide_from.unwrap_or(self.target);

        let d = |p: P| p.def().sanitize(patch[p as usize]);
        self.unison = [d(P::Osc1Unison) as usize, d(P::Osc2Unison) as usize];
        self.filter_mode = match d(P::FilterType) as u8 {
            1 => Mode::Band,
            2 => Mode::High,
            _ => Mode::Low,
        };
        self.steep = d(P::FilterSlope) >= 18.0;
        self.sub_octave = d(P::SubOctave);

        if was_silent {
            for (b, (rp, bank)) in [P::Osc1Phase, P::Osc2Phase]
                .into_iter()
                .zip(self.osc.iter_mut())
                .enumerate()
            {
                let random = patch[rp as usize].clamp(0.0, 1.0);
                for (i, o) in bank.iter_mut().enumerate() {
                    // Copies never start in step even without randomness,
                    // or unison would sound like one loud oscillator.
                    let spread = if i == 0 {
                        0.0
                    } else {
                        i as f32 * 0.137 + b as f32 * 0.31
                    };
                    o.reset(spread + random * self.noise.unit());
                }
            }
            self.sub_phase = 0.0;
            for s in &mut self.shaper {
                s.clear();
            }
            for d in &mut self.drive {
                d.clear();
            }
            for f in self.filter.iter_mut().flatten() {
                f.clear();
            }
            self.fm_prev = 0.0;
        }
        self.amp.gate_on();
        self.fenv.gate_on();
        self.menv.gate_on();
        self.ctl_left = 0;
    }

    /// Release the note: envelopes go to their release stage.
    pub fn release(&mut self) {
        self.held = false;
        self.amp.gate_off();
        self.fenv.gate_off();
        self.menv.gate_off();
    }

    /// Refresh the live values from the patch and recompute what changes
    /// at control rate.
    fn control(&mut self, patch: &[f32]) {
        self.live.copy_from_slice(patch);
        let v = &self.live;
        let g = |p: P| v[p as usize];
        let ids = [
            (
                P::Osc1Level,
                P::Osc1Shape,
                P::Osc1Pw,
                P::Osc1Octave,
                P::Osc1Semi,
                P::Osc1Fine,
                P::Osc1Detune,
                P::Osc1Spread,
                P::Osc1Sync,
            ),
            (
                P::Osc2Level,
                P::Osc2Shape,
                P::Osc2Pw,
                P::Osc2Octave,
                P::Osc2Semi,
                P::Osc2Fine,
                P::Osc2Detune,
                P::Osc2Spread,
                P::Osc2Sync,
            ),
        ];
        for (b, (lv, sh, pw, oc, se, fi, de, sp, sy)) in ids.into_iter().enumerate() {
            let n = self.unison[b];
            let bank = &mut self.banks[b];
            bank.level = g(lv);
            bank.shape = Shape::new(g(sh), g(pw));
            bank.sync = g(sy).max(1.0);
            let base = 12.0 * g(oc).round() + g(se).round() + g(fi) / 100.0;
            let norm = 1.0 / (n as f32).sqrt();
            for i in 0..n {
                // Copies spread evenly over -1..1; the middle one (odd
                // counts) sits in tune and in the centre.
                let t = if n == 1 {
                    0.0
                } else {
                    2.0 * i as f32 / (n - 1) as f32 - 1.0
                };
                let semis = base + t * g(de) / 100.0 * 0.5;
                bank.ratio[i] = (semis / 12.0).exp2();
                let pan = (t * g(sp)).clamp(-1.0, 1.0);
                let a = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
                bank.gain[i] = (a.cos() * norm, a.sin() * norm);
            }
        }
    }

    /// Render `left.len()` samples, adding into the buffers.
    pub fn render(&mut self, patch: &[f32], glide_coef: f32, left: &mut [f32], right: &mut [f32]) {
        let sr = self.sr;
        let inv_sr = 1.0 / sr;
        for i in 0..left.len() {
            if self.ctl_left == 0 {
                self.control(patch);
                self.ctl_left = CONTROL_BLOCK;
            }
            self.ctl_left -= 1;
            let v = &self.live;
            let g = |p: P| v[p as usize];

            let a = self.amp.next(
                g(P::AmpAttack),
                g(P::AmpDecay),
                g(P::AmpSustain),
                g(P::AmpRelease),
                sr,
            );
            let fe = self.fenv.next(
                g(P::FenvAttack),
                g(P::FenvDecay),
                g(P::FenvSustain),
                g(P::FenvRelease),
                sr,
            );
            self.menv.next(
                g(P::MenvAttack),
                g(P::MenvDecay),
                g(P::MenvSustain),
                g(P::MenvRelease),
                sr,
            );
            if self.amp.is_idle() {
                self.note = None;
                break;
            }

            self.pitch += (self.target - self.pitch) * glide_coef;
            let hz = 440.0 * ((self.pitch - 69.0) / 12.0).exp2();
            let inc = hz * inv_sr;

            // Oscillator 2 first: it is the FM modulator.
            let (mut l, mut r) = (0.0f32, 0.0f32);
            let mut mono2 = 0.0;
            let b2 = self.banks[1];
            if b2.level > 0.0 || g(P::FmAmount) > 0.0 {
                for k in 0..self.unison[1] {
                    let s = self.osc[1][k].next(&b2.shape, inc * b2.ratio[k], b2.sync);
                    mono2 += s * (b2.gain[k].0 + b2.gain[k].1) * 0.5;
                    l += s * b2.gain[k].0 * b2.level;
                    r += s * b2.gain[k].1 * b2.level;
                }
            }
            let fm = 1.0 + g(P::FmAmount) * self.fm_prev;
            self.fm_prev = mono2 * std::f32::consts::SQRT_2;
            let b1 = self.banks[0];
            if b1.level > 0.0 {
                for k in 0..self.unison[0] {
                    let s = self.osc[0][k].next(&b1.shape, (inc * b1.ratio[k] * fm).abs(), b1.sync);
                    l += s * b1.gain[k].0 * b1.level;
                    r += s * b1.gain[k].1 * b1.level;
                }
            }
            let sub = g(P::SubLevel);
            if sub > 0.0 {
                self.sub_phase = (self.sub_phase + inc * self.sub_octave.exp2()).fract();
                let s = (std::f32::consts::TAU * self.sub_phase).sin() * sub * 0.707;
                l += s;
                r += s;
            }
            let noise = g(P::NoiseLevel);
            if noise > 0.0 {
                // Two independent channels: noise has no single position.
                l += self.noise.white() * noise * 0.5;
                r += self.noise.white() * noise * 0.5;
            }

            let (fold, drive) = (g(P::Fold), g(P::Drive));
            let [dl, dr] = &mut self.drive;
            l = self.shaper[0].process(l, |x| dl.process(dsp::fold(x, fold), drive));
            r = self.shaper[1].process(r, |x| dr.process(dsp::fold(x, fold), drive));

            let track = ((self.pitch - 60.0) / 12.0 * g(P::Keytrack)).exp2();
            let cutoff = g(P::Cutoff) * track * (g(P::FilterEnv) * fe).exp2();
            let res = g(P::Resonance);
            let mode = self.filter_mode;
            if self.steep {
                let c1 = Coefs::new(cutoff * inv_sr, 1.848);
                let c2 = Coefs::new(cutoff * inv_sr, dsp::damping(res, 0.765));
                let [fl, fr] = &mut self.filter;
                let (lm, rm) = (fl[0].process(l, &c1, mode), fr[0].process(r, &c1, mode));
                l = fl[1].process(lm, &c2, mode);
                r = fr[1].process(rm, &c2, mode);
            } else {
                let c = Coefs::new(cutoff * inv_sr, dsp::damping(res, std::f32::consts::SQRT_2));
                l = self.filter[0][0].process(l, &c, mode);
                r = self.filter[1][0].process(r, &c, mode);
            }

            let vs = g(P::Velocity);
            let gain = a * (1.0 - vs + vs * self.velocity);
            left[i] += l * gain;
            right[i] += r * gain;
        }
    }
}
