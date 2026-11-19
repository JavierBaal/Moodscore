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

use crate::params::{self, P, PARAMS};
use crate::voice::{Ctx, Voice};

/// Most notes a part can sound at once.
pub const MAX_VOICES: usize = 4;

/// Keys remembered by a monophonic part.
const STACK: usize = 16;

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
}

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
        }
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
        let glide = self.glide_from();
        let v = &mut self.voices[0];
        if legato && v.held && v.is_active() {
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

    /// Render `left.len()` samples, adding into the buffers. Each event is
    /// applied at its sample offset in the block (offsets past the end are
    /// applied at the end).
    pub fn render(&mut self, events: &[(usize, Event)], left: &mut [f32], right: &mut [f32]) {
        let len = left.len();
        let mut at = 0;
        let mut i = 0;
        while at < len || i < events.len() {
            let next = events.get(i).map_or(len, |&(t, _)| t.min(len));
            if next > at {
                self.render_span(&mut left[at..next], &mut right[at..next]);
                at = next;
            }
            if let Some(&(_, e)) = events.get(i) {
                self.handle(e);
                i += 1;
            } else {
                break;
            }
        }
    }

    fn render_span(&mut self, left: &mut [f32], right: &mut [f32]) {
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
        for v in &mut self.voices {
            if v.is_active() {
                v.render(&self.patch, &ctx, left, right);
            }
        }
        // Free-running LFO phases move on whether or not anything sounds.
        let dt = left.len() as f32 / self.sr;
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
