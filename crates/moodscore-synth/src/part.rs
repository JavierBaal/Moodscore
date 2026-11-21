//! A part: one patch played by up to [`MAX_VOICES`] voices.
//!
//! Polyphonic, a new note takes a free voice or steals the oldest (which
//! rises from wherever it is, so stealing never clicks). Monophonic
//! (`part/voices` = 1), the part keeps a stack of held keys: with legato,
//! overlapping notes slide without restarting the envelopes, and letting
//! go of a key slides back to the one still held.
//!
//! Notes arrive as [`Event`]s stamped with the sample they belong to;
//! [`Part::render`] splits its block there, so timing is sample-accurate.
//! Nothing here allocates after [`Part::new`].

use crate::fx::{Fx, FxParams};
use crate::params::{self, P, PARAMS};
use crate::voice::{Ctx, Voice};

/// Most notes a part can sound at once.
pub const MAX_VOICES: usize = 4;

/// Keys remembered by a monophonic part.
const STACK: usize = 16;

/// Samples processed at a time inside the part.
const SPAN: usize = 256;

/// Where a part writes: its dry sound and its reverb send, stereo.
pub struct Bus<'a> {
    pub dry_l: &'a mut [f32],
    pub dry_r: &'a mut [f32],
    pub send_l: &'a mut [f32],
    pub send_r: &'a mut [f32],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Event {
    /// MIDI note and velocity (`0..1`).
    NoteOn {
        note: u8,
        velocity: f32,
    },
    NoteOff {
        note: u8,
    },
    /// Release every note.
    AllOff,
}

pub struct Part {
    sr: f32,
    patch: Vec<f32>,
    voices: Vec<Voice>,
    clock: u64,
    /// The pitch of the last note started, to glide from.
    last_pitch: Option<f32>,
    /// Held keys, oldest first (monophonic mode).
    stack: [u8; STACK],
    depth: usize,
    /// Running phase of each LFO, for LFOs that don't restart with notes.
    free_phase: [f32; 2],
    quarters_per_second: f32,
    fx: Fx,
    scratch: [[f32; SPAN]; 2],
    /// Part gains (left, right, send) last applied, ramped per span.
    gains: Option<[f32; 3]>,
    /// Frames the part has been silent with no note sounding.
    quiet: usize,
    /// At rest: no note sounds and the effects have died away, so nothing
    /// is computed until the next note (a waiting part costs next to
    /// nothing).
    resting: bool,
    /// What the part puts out (its dry sound, mono), band by band.
    meter: crate::meter::BandMeter,
}

/// Below this the part's output counts as silence (-120 dB).
const SILENT: f32 = 1e-6;

/// The shortest silence before a part rests, in seconds (its echoes may
/// need longer: see `render_span`).
const REST_AFTER: f32 = 0.5;

impl Part {
    /// Allocates: call before audio starts.
    pub fn new(sample_rate: u32, seed: u32) -> Self {
        Self {
            sr: sample_rate as f32,
            patch: PARAMS.iter().map(|p| p.init).collect(),
            voices: (0..MAX_VOICES as u32)
                .map(|i| Voice::new(sample_rate, seed.wrapping_add(i * 7919)))
                .collect(),
            clock: 0,
            last_pitch: None,
            stack: [0; STACK],
            depth: 0,
            free_phase: [0.0; 2],
            quarters_per_second: 2.0,
            fx: Fx::new(sample_rate),
            scratch: [[0.0; SPAN]; 2],
            gains: None,
            quiet: 0,
            resting: true,
            meter: crate::meter::BandMeter::new(sample_rate as f32),
        }
    }

    /// The energy the part has put out since the last call, in the meter's
    /// three bands (see [`crate::meter`]).
    pub fn take_meter(&mut self) -> [f32; 3] {
        self.meter.take()
    }

    /// True while nothing sounds and the effects have died away.
    pub fn is_resting(&self) -> bool {
        self.resting
    }

    pub fn patch(&self) -> &[f32] {
        &self.patch
    }

    /// Set a patch parameter (clamped; discrete ones rounded). Continuous
    /// ones apply to sounding notes at the next control block, discrete
    /// ones from the next note.
    pub fn set(&mut self, id: usize, value: f32) {
        if let Some(def) = PARAMS.get(id) {
            self.patch[id] = def.sanitize(value);
        }
    }

    /// Set the whole patch.
    pub fn set_patch(&mut self, values: &[f32]) {
        for (id, &v) in values.iter().enumerate().take(PARAMS.len()) {
            self.set(id, v);
        }
    }

    /// Tempo for synced LFOs.
    pub fn set_tempo(&mut self, quarters_per_second: f32) {
        self.quarters_per_second = quarters_per_second.max(1e-3);
    }

    /// Notes sounding (held or releasing).
    pub fn active_voices(&self) -> usize {
        self.voices.iter().filter(|v| v.is_active()).count()
    }

    fn get(&self, p: P) -> f32 {
        p.def().sanitize(self.patch[p as usize])
    }

    fn polyphony(&self) -> usize {
        (self.get(P::PartVoices) as usize).clamp(1, MAX_VOICES)
    }

    fn glide_from(&self) -> Option<f32> {
        (self.get(P::PartGlide) > 0.0)
            .then_some(self.last_pitch)
            .flatten()
    }

    pub fn handle(&mut self, event: Event) {
        match event {
            Event::NoteOn { note, velocity } => self.note_on(note.min(127), velocity),
            Event::NoteOff { note } => self.note_off(note),
            Event::AllOff => {
                self.depth = 0;
                for v in &mut self.voices {
                    v.release();
                }
            }
        }
    }

    fn note_on(&mut self, note: u8, velocity: f32) {
        self.clock += 1;
        let n = self.polyphony();
        // Voices beyond the polyphony (it was lowered) let go.
        for v in &mut self.voices[n..] {
            if v.held {
                v.release();
            }
        }
        if n == 1 {
            return self.mono_on(note, velocity);
        }
        let glide = self.glide_from();
        let idx = self.voices[..n]
            .iter()
            .position(|v| v.is_active() && v.note == Some(note))
            .or_else(|| self.voices[..n].iter().position(|v| !v.is_active()))
            .unwrap_or_else(|| {
                // Steal: the oldest released note, else the oldest held one.
                let oldest = |held: bool| {
                    self.voices[..n]
                        .iter()
                        .enumerate()
                        .filter(|(_, v)| v.held == held)
                        .min_by_key(|(_, v)| v.age)
                        .map(|(i, _)| i)
                };
                oldest(false).or_else(|| oldest(true)).unwrap_or(0)
            });
        let v = &mut self.voices[idx];
        v.start(&self.patch, note, velocity, glide, self.free_phase);
        v.age = self.clock;
        self.last_pitch = Some(note as f32);
    }

    fn mono_on(&mut self, note: u8, velocity: f32) {
        self.remove_from_stack(note);
        if self.depth == STACK {
            self.stack.copy_within(1.., 0);
            self.depth -= 1;
        }
        self.stack[self.depth] = note;
        self.depth += 1;

        let legato = self.get(P::PartLegato) >= 0.5;
        let overlapping = self.voices[0].held && self.voices[0].is_active();
        // Legato glides only between overlapping notes (fingered
        // portamento): a detached note starts on its own pitch.
        let glide = if legato && !overlapping {
            None
        } else {
            self.glide_from()
        };
        let v = &mut self.voices[0];
        if legato && overlapping {
            v.retarget(note);
            if glide.is_none() {
                v.pitch = note as f32;
            }
        } else {
            v.start(&self.patch, note, velocity, glide, self.free_phase);
        }
        v.age = self.clock;
        self.last_pitch = Some(note as f32);
    }

    fn remove_from_stack(&mut self, note: u8) {
        if let Some(i) = self.stack[..self.depth].iter().position(|&k| k == note) {
            self.stack.copy_within(i + 1..self.depth, i);
            self.depth -= 1;
        }
    }

    fn note_off(&mut self, note: u8) {
        if self.polyphony() == 1 {
            let was_top = self.depth > 0 && self.stack[self.depth - 1] == note;
            self.remove_from_stack(note);
            let no_glide = self.get(P::PartGlide) <= 0.0;
            let v = &mut self.voices[0];
            if !was_top || v.note != Some(note) {
                return;
            }
            if self.depth > 0 {
                // Back to the key still held, without restarting.
                let back = self.stack[self.depth - 1];
                v.retarget(back);
                if no_glide {
                    v.pitch = back as f32;
                }
                self.last_pitch = Some(back as f32);
            } else {
                v.release();
            }
            return;
        }
        for v in &mut self.voices {
            if v.held && v.note == Some(note) {
                v.release();
            }
        }
    }

    /// Render the dry sound only (the send is dropped), adding into the
    /// buffers. See [`render_bus`](Self::render_bus).
    pub fn render(&mut self, events: &[(usize, Event)], left: &mut [f32], right: &mut [f32]) {
        let mut sl = [0.0f32; SPAN];
        let mut sr = [0.0f32; SPAN];
        let len = left.len();
        let mut start = 0;
        let mut ev = 0;
        while start < len || ev < events.len() {
            let end = (start + SPAN).min(len);
            let first = ev;
            while ev < events.len() && (events[ev].0 < end || end == len) {
                ev += 1;
            }
            let n = end - start;
            sl[..n].fill(0.0);
            sr[..n].fill(0.0);
            let mut local = [(0usize, Event::AllOff); 64];
            let mut count = 0;
            for &(t, e) in &events[first..ev] {
                if count < local.len() {
                    local[count] = (t.saturating_sub(start), e);
                    count += 1;
                }
            }
            self.render_bus(
                &local[..count],
                Bus {
                    dry_l: &mut left[start..end],
                    dry_r: &mut right[start..end],
                    send_l: &mut sl[..n],
                    send_r: &mut sr[..n],
                },
            );
            if end == len {
                break;
            }
            start = end;
        }
    }

    /// Render `bus.dry_l.len()` samples, adding the dry sound and the reverb
    /// send into the bus. Each event is applied at its sample offset in the
    /// block (offsets past the end are applied at the end).
    pub fn render_bus(&mut self, events: &[(usize, Event)], bus: Bus) {
        let Bus {
            dry_l,
            dry_r,
            send_l,
            send_r,
        } = bus;
        let len = dry_l.len();
        let mut at = 0;
        let mut i = 0;
        while at < len || i < events.len() {
            let next = events.get(i).map_or(len, |&(t, _)| t.min(len));
            while next > at {
                let end = next.min(at + SPAN);
                self.render_span(
                    &mut dry_l[at..end],
                    &mut dry_r[at..end],
                    &mut send_l[at..end],
                    &mut send_r[at..end],
                );
                at = end;
            }
            if let Some(&(_, e)) = events.get(i) {
                self.handle(e);
                i += 1;
            } else {
                break;
            }
        }
    }

    fn render_span(
        &mut self,
        dry_l: &mut [f32],
        dry_r: &mut [f32],
        send_l: &mut [f32],
        send_r: &mut [f32],
    ) {
        let n = dry_l.len();
        let sounding = self.voices.iter().any(|v| v.is_active());
        let fxp = self.fx_params();
        if !sounding && self.resting {
            self.fx.idle(&fxp, n);
            self.advance_lfos(n);
            return;
        }
        self.resting = false;
        let glide = self.get(P::PartGlide);
        // Glide reaches ~95% of the way in the glide time.
        let glide_coef = if glide > 0.0 {
            1.0 - (-3.0 / (glide * self.sr)).exp()
        } else {
            1.0
        };
        let ctx = Ctx {
            glide_coef,
            quarters_per_second: self.quarters_per_second,
        };
        let [sl, sr] = &mut self.scratch;
        let (left, right) = (&mut sl[..n], &mut sr[..n]);
        left.fill(0.0);
        right.fill(0.0);
        for v in &mut self.voices {
            if v.is_active() {
                v.render(&self.patch, &ctx, left, right);
            }
        }

        let g = |p: P| self.patch[p as usize];
        self.fx.process(&fxp, left, right);
        // Rest once nothing sounds and the effects are silent for longer
        // than an echo takes to come back.
        if sounding {
            self.quiet = 0;
        } else {
            let peak = left
                .iter()
                .chain(right.iter())
                .fold(0.0f32, |m, x| m.max(x.abs()));
            self.quiet = if peak < SILENT { self.quiet + n } else { 0 };
            let wait = REST_AFTER.max(1.5 * fxp.delay_time) * self.sr;
            if self.quiet as f32 >= wait {
                self.resting = true;
                self.quiet = 0;
            }
        }

        let (level, pan, send) = (g(P::PartLevel), g(P::PartPan), g(P::ReverbSend));
        let to = [
            level * (1.0 - pan).min(1.0),
            level * (1.0 + pan).min(1.0),
            send,
        ];
        let from = self.gains.unwrap_or(to);
        self.gains = Some(to);
        for i in 0..n {
            let k = (i + 1) as f32 / n as f32;
            let gl = from[0] + (to[0] - from[0]) * k;
            let gr = from[1] + (to[1] - from[1]) * k;
            let gs = from[2] + (to[2] - from[2]) * k;
            let (l, r) = (left[i] * gl, right[i] * gr);
            self.meter.feed(0.5 * (l + r));
            dry_l[i] += l;
            dry_r[i] += r;
            send_l[i] += l * gs;
            send_r[i] += r * gs;
        }
        self.advance_lfos(n);
    }

    fn fx_params(&self) -> FxParams {
        let g = |p: P| self.patch[p as usize];
        FxParams {
            distortion: g(P::Distortion),
            ensemble: g(P::Ensemble),
            chorus_mix: g(P::ChorusMix),
            chorus_rate: g(P::ChorusRate),
            chorus_depth: g(P::ChorusDepth),
            delay_mix: g(P::DelayMix),
            delay_time: params::DIVISION_BEATS
                [P::DelayDivision.def().sanitize(g(P::DelayDivision)) as usize]
                / self.quarters_per_second,
            delay_feedback: g(P::DelayFeedback),
            delay_width: g(P::DelayWidth),
            delay_tone: g(P::DelayTone),
        }
    }

    /// Free-running LFO phases move on whether or not anything sounds.
    fn advance_lfos(&mut self, n: usize) {
        let dt = n as f32 / self.sr;
        for (k, (rate, sync, div)) in [
            (P::Lfo1Rate, P::Lfo1Sync, P::Lfo1Division),
            (P::Lfo2Rate, P::Lfo2Sync, P::Lfo2Division),
        ]
        .into_iter()
        .enumerate()
        {
            let hz = if self.get(sync) >= 0.5 {
                self.quarters_per_second / params::DIVISION_BEATS[self.get(div) as usize]
            } else {
                self.get(rate)
            };
            self.free_phase[k] = (self.free_phase[k] + hz * dt).fract();
        }
    }
}
