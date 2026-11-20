//! One sounding note: two unison oscillator banks, sub, noise, the
//! oversampled shaper, the filter and three envelopes.

use crate::dsp::{self, Coefs, Drive, Env, Ladder, LadderCoefs, Mode, Oversampler, Svf};
use crate::lfo::Lfo;
use crate::osc::{Noise, Osc, Shape};
use crate::params::{DIVISION_BEATS, MOD_DESTS, MOD_SLOTS, P, PARAMS};

/// What a voice needs from its part while it renders.
#[derive(Copy, Clone, Debug)]
pub struct Ctx {
    /// Per-sample glide coefficient (1 = no glide).
    pub glide_coef: f32,
    /// The tempo, for synced LFOs.
    pub quarters_per_second: f32,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            glide_coef: 1.0,
            quarters_per_second: 2.0,
        }
    }
}

const N_DESTS: usize = MOD_DESTS.len();

/// Parameter ids of the modulation destinations, in the order of [`MOD_DESTS`].
fn dest_ids() -> [usize; N_DESTS] {
    let mut ids = [usize::MAX; N_DESTS];
    for (i, d) in MOD_DESTS.iter().enumerate().skip(1) {
        ids[i] = crate::params::find(d).unwrap_or(usize::MAX);
    }
    ids
}

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
    ladder: bool,
    ladders: [Ladder; 2],
    analog: Analog,
    sub_octave: f32,
    lfo: [Lfo; 2],
    /// Random per note, `-1..1`: a modulation source.
    random: f32,
    dests: [usize; N_DESTS],
    /// Output gains (left, right) at the start and end of this control block.
    gain_from: (f32, f32),
    gain_to: (f32, f32),
    fresh: bool,
    /// The note held, and whether its key is still down.
    pub(crate) note: Option<u8>,
    pub(crate) held: bool,
    /// When the note started, for stealing the oldest.
    pub(crate) age: u64,
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
            ladder: false,
            ladders: Default::default(),
            analog: Analog::new(seed),
            sub_octave: -1.0,
            lfo: [Lfo::new(seed ^ 0x9e37), Lfo::new(seed ^ 0x7f4a)],
            random: 0.0,
            dests: dest_ids(),
            gain_from: (1.0, 1.0),
            gain_to: (1.0, 1.0),
            fresh: true,
            note: None,
            held: false,
            age: 0,
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
    ///
    /// `glide_from` is the pitch to slide from; `free_phase` the running
    /// phase of each LFO for those that don't restart with the note.
    pub fn start(
        &mut self,
        patch: &[f32],
        note: u8,
        velocity: f32,
        glide_from: Option<f32>,
        free_phase: [f32; 2],
    ) {
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
        self.ladder = d(P::FilterType) as u8 == 3;
        self.sub_octave = d(P::SubOctave);
        self.analog.new_note();

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
            for l in &mut self.ladders {
                l.clear();
            }
            self.fm_prev = 0.0;
            self.fresh = true;
        }
        self.random = self.noise.white();
        for (k, (lfo, rt)) in self
            .lfo
            .iter_mut()
            .zip([P::Lfo1Retrigger, P::Lfo2Retrigger])
            .enumerate()
        {
            let retrigger = rt.def().sanitize(patch[rt as usize]) >= 0.5;
            lfo.start(if retrigger { 0.0 } else { free_phase[k] });
        }
        self.amp.gate_on();
        self.fenv.gate_on();
        self.menv.gate_on();
        self.ctl_left = 0;
    }

    /// Slide to another note without restarting anything (legato).
    pub fn retarget(&mut self, note: u8) {
        self.note = Some(note);
        self.held = true;
        self.target = note as f32;
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
    fn control(&mut self, patch: &[f32], ctx: &Ctx) {
        let dt = CONTROL_BLOCK as f32 / self.sr;
        // LFOs move at the rate of the last block (their rate can be modulated).
        let mut lfo = [0.0f32; 2];
        for (k, (shape, rate, sync, div, delay, fade)) in [
            (
                P::Lfo1Shape,
                P::Lfo1Rate,
                P::Lfo1Sync,
                P::Lfo1Division,
                P::Lfo1Delay,
                P::Lfo1Fade,
            ),
            (
                P::Lfo2Shape,
                P::Lfo2Rate,
                P::Lfo2Sync,
                P::Lfo2Division,
                P::Lfo2Delay,
                P::Lfo2Fade,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let d = |p: P| p.def().sanitize(patch[p as usize]);
            let hz = if d(sync) >= 0.5 {
                ctx.quarters_per_second / DIVISION_BEATS[d(div) as usize]
            } else {
                self.live[rate as usize]
            };
            lfo[k] = self.lfo[k].advance(dt, hz, d(shape) as u8, d(delay), d(fade));
        }

        self.live.copy_from_slice(patch);
        self.analog
            .advance(dt, patch[P::Analog as usize].clamp(0.0, 1.0));
        let sources = [
            0.0,
            self.menv.level,
            self.fenv.level,
            self.amp.level,
            lfo[0],
            lfo[1],
            self.velocity,
            ((self.pitch - 60.0) / 48.0).clamp(-1.0, 1.0),
            self.random,
            patch[P::Expression as usize],
        ];
        for slot in 0..MOD_SLOTS {
            let base = P::Mod1Source as usize + 3 * slot;
            let src = PARAMS[base].sanitize(patch[base]) as usize;
            let dst = PARAMS[base + 1].sanitize(patch[base + 1]) as usize;
            let amount = patch[base + 2];
            if src == 0 || dst == 0 || amount == 0.0 {
                continue;
            }
            let id = self.dests[dst];
            let def = &PARAMS[id];
            let n = def.normalize(self.live[id]) + amount * sources[src];
            self.live[id] = def.denormalize(n);
        }

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
                let semis = base + t * g(de) / 100.0 * 0.5 + self.analog.copy_cents(b, i) / 100.0;
                bank.ratio[i] = (semis / 12.0).exp2();
                let pan = (t * g(sp)).clamp(-1.0, 1.0);
                let a = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
                bank.gain[i] = (a.cos() * norm, a.sin() * norm);
            }
        }
        let gain = g(P::VoiceGain);
        let pan = g(P::VoicePan);
        let to = (gain * (1.0 - pan).min(1.0), gain * (1.0 + pan).min(1.0));
        // A fresh note starts at its gain; a sounding one ramps to it.
        self.gain_from = if self.fresh { to } else { self.gain_to };
        self.gain_to = to;
        self.fresh = false;
    }

    /// Render `left.len()` samples, adding into the buffers.
    pub fn render(&mut self, patch: &[f32], ctx: &Ctx, left: &mut [f32], right: &mut [f32]) {
        let sr = self.sr;
        let inv_sr = 1.0 / sr;
        for i in 0..left.len() {
            if self.ctl_left == 0 {
                self.control(patch, ctx);
                self.ctl_left = CONTROL_BLOCK;
            }
            self.ctl_left -= 1;
            let k = 1.0 - self.ctl_left as f32 / CONTROL_BLOCK as f32;
            let out_l = self.gain_from.0 + (self.gain_to.0 - self.gain_from.0) * k;
            let out_r = self.gain_from.1 + (self.gain_to.1 - self.gain_from.1) * k;
            let v = &self.live;
            let g = |p: P| v[p as usize];

            let [ta, td, tr] = self.analog.env_scale;
            let a = self.amp.next(
                g(P::AmpAttack) * ta,
                g(P::AmpDecay) * td,
                g(P::AmpSustain),
                g(P::AmpRelease) * tr,
                sr,
            );
            let fe = self.fenv.next(
                g(P::FenvAttack) * ta,
                g(P::FenvDecay) * td,
                g(P::FenvSustain),
                g(P::FenvRelease) * tr,
                sr,
            );
            self.menv.next(
                g(P::MenvAttack) * ta,
                g(P::MenvDecay) * td,
                g(P::MenvSustain),
                g(P::MenvRelease) * tr,
                sr,
            );
            if self.amp.is_idle() {
                self.note = None;
                break;
            }

            self.pitch += (self.target - self.pitch) * ctx.glide_coef;
            let detune = self.analog.cents / 100.0;
            let hz = 440.0 * ((self.pitch + g(P::Bend) + detune - 69.0) / 12.0).exp2();
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
            let cutoff =
                g(P::Cutoff) * track * (g(P::FilterEnv) * fe + self.analog.cutoff_octaves).exp2();
            let res = g(P::Resonance);
            let mode = self.filter_mode;
            if self.ladder {
                let c = LadderCoefs::new(cutoff * inv_sr, res, g(P::FilterDrive));
                l = self.ladders[0].process(l, &c);
                r = self.ladders[1].process(r, &c);
            } else if self.steep {
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
            left[i] += l * gain * out_l;
            right[i] += r * gain * out_r;
        }
    }
}

/// What makes each voice a little different, as on an analog synth: a fixed
/// detune per note (and per unison copy), a slow wander, a different cutoff
/// and slightly different envelope times. Scaled by `part/analog`; at 0 it
/// does nothing. It has its own random source, so it never changes the
/// rest of the voice's randomness.
#[derive(Clone, Debug)]
struct Analog {
    rng: Noise,
    depth: f32,
    // Per note, unit randoms (-1..1).
    note: f32,
    copies: [[f32; MAX_UNISON]; 2],
    cutoff: f32,
    env: [f32; 3],
    // The wander: a smooth path between random points.
    from: f32,
    to: f32,
    phase: f32,
    rate: f32,
    /// Results, refreshed each control block.
    cents: f32,
    cutoff_octaves: f32,
    env_scale: [f32; 3],
}

/// At `part/analog` = 1: cents of fixed detune, cents of wander, cents
/// between unison copies, octaves of cutoff spread, share of envelope time.
const DETUNE_CENTS: f32 = 6.0;
const WANDER_CENTS: f32 = 4.0;
const COPY_CENTS: f32 = 3.0;
const CUTOFF_OCTAVES: f32 = 0.25;
const ENV_SHARE: f32 = 0.12;

impl Analog {
    fn new(seed: u32) -> Self {
        let mut rng = Noise::new(seed.wrapping_mul(0x9E37_79B9) ^ 0x51ED_270B);
        let to = rng.white();
        let rate = 0.3 + 0.6 * rng.unit();
        Self {
            rng,
            depth: 0.0,
            note: 0.0,
            copies: [[0.0; MAX_UNISON]; 2],
            cutoff: 0.0,
            env: [0.0; 3],
            from: 0.0,
            to,
            phase: 0.0,
            rate,
            cents: 0.0,
            cutoff_octaves: 0.0,
            env_scale: [1.0; 3],
        }
    }

    fn new_note(&mut self) {
        self.note = self.rng.white();
        for bank in &mut self.copies {
            for c in bank.iter_mut() {
                *c = self.rng.white();
            }
        }
        self.cutoff = self.rng.white();
        for e in &mut self.env {
            *e = self.rng.white();
        }
    }

    /// Move the wander on by `dt` seconds and refresh the results at `depth`.
    fn advance(&mut self, dt: f32, depth: f32) {
        self.depth = depth;
        self.phase += dt * self.rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.from = self.to;
            self.to = self.rng.white();
            self.rate = 0.3 + 0.6 * self.rng.unit();
        }
        let p = self.phase;
        let smooth = p * p * (3.0 - 2.0 * p);
        let wander = self.from + (self.to - self.from) * smooth;
        self.cents = depth * (DETUNE_CENTS * self.note + WANDER_CENTS * wander);
        self.cutoff_octaves = depth * CUTOFF_OCTAVES * self.cutoff;
        for (s, e) in self.env_scale.iter_mut().zip(self.env) {
            *s = 1.0 + depth * ENV_SHARE * e;
        }
    }

    /// Extra cents for unison copy `i` of bank `b`.
    fn copy_cents(&self, b: usize, i: usize) -> f32 {
        self.depth * COPY_CENTS * self.copies[b][i]
    }
}
