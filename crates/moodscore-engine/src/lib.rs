//! Moodscore audio engine.
//!
//! The engine is split in two halves that share only atomics:
//!
//! - [`Renderer`] lives on the audio thread. Every few milliseconds it reads
//!   the parameter targets, moves each parameter a little toward its target
//!   (so nothing ever jumps), runs the DSP and updates the level meters.
//!   It never allocates, locks, logs or waits.
//! - [`EngineHandle`] lives on control threads. It sets targets and reads the
//!   meters. Cloning it is cheap.
//!
//! [`audio::start_default_output`] wires a [`Renderer`] to the system's
//! default audio device.
//!
//! The parameters are the Faust DSP's (the drones and the room) followed by
//! the two note parts' (`note_1/…`, `note_2/…`, from `moodscore-synth`).
//! Notes travel to the audio thread through a lock-free queue, each stamped
//! with the frame it belongs to, and start on that exact sample.

pub mod audio;

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use moodscore_dsp::Dsp;
pub use moodscore_dsp::{ParamInfo, Scale};
pub use moodscore_synth::{params as synth_params, Event as NoteEvent};
use moodscore_synth::{Bus, Part};

/// Number of note parts.
pub const PARTS: usize = 2;

/// Notes that can wait in the queue for the audio thread.
const NOTE_QUEUE: usize = 1024;

/// A note event for a part, due at an absolute frame.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NoteMsg {
    pub part: usize,
    pub frame: u64,
    pub event: NoteEvent,
}

/// The path prefix of a part's parameters: `note_1`, `note_2`.
pub fn part_prefix(part: usize) -> String {
    format!("note_{}", part + 1)
}

/// Frames rendered between parameter updates (≈0.7 ms at 48 kHz).
pub const CHUNK: usize = 32;

/// Time constant of the engine's parameter smoothing, in seconds.
///
/// This is the safety net under the director's gestures: even a target that
/// moves instantly reaches the DSP as a short glide, never a step.
pub const SMOOTHING_SECONDS: f32 = 0.03;

/// Fade-in applied when the engine starts, in seconds, unless the output
/// is told otherwise with [`EngineHandle::fade_to`] before audio starts.
pub const FADE_IN_SECONDS: f32 = 1.5;

/// The shortest output fade, in seconds. Anything faster would click.
pub const MIN_FADE_SECONDS: f32 = 0.05;

/// Integration time of the RMS meter, in seconds.
const RMS_SECONDS: f32 = 0.3;
/// Peak hold: the peak meter falls by about 8.7 dB per this many seconds
/// (exponential release), like any ordinary peak meter.
const PEAK_RELEASE_SECONDS: f32 = 1.0;

/// State shared between the audio thread and control threads. Atomics only.
struct Shared {
    targets: Box<[AtomicU32]>,
    peak: AtomicU32,
    rms: AtomicU32,
    frames: AtomicU64,
    /// Output gain the engine is fading toward (0 = silent, 1 = full).
    fade_target: AtomicU32,
    /// Fade speed, in gain per sample. Always positive.
    fade_step: AtomicU32,
    /// Current output gain, as last rendered.
    fade_current: AtomicU32,
    /// Tempo for the note parts (synced LFOs and echoes), quarters per second.
    tempo: AtomicU32,
    /// Notes sounding in each part.
    voices: [AtomicU32; PARTS],
}

impl Shared {
    fn store_f32(cell: &AtomicU32, v: f32) {
        cell.store(v.to_bits(), Ordering::Relaxed);
    }
    fn load_f32(cell: &AtomicU32) -> f32 {
        f32::from_bits(cell.load(Ordering::Relaxed))
    }
}

/// Output levels as measured after the DSP.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Levels {
    /// Peak with a 1 s release, linear (0..1). Never below the RMS of a
    /// steady signal, unlike the peak of a single short chunk.
    pub peak: f32,
    /// RMS over roughly the last 300 ms, linear (0..1).
    pub rms: f32,
}

impl Levels {
    pub fn peak_dbfs(&self) -> f32 {
        to_dbfs(self.peak)
    }
    pub fn rms_dbfs(&self) -> f32 {
        to_dbfs(self.rms)
    }
}

fn to_dbfs(x: f32) -> f32 {
    if x <= 1e-6 {
        -120.0
    } else {
        20.0 * x.log10()
    }
}

/// Control-side handle to a running engine. Cheap to clone; never blocks.
#[derive(Clone)]
pub struct EngineHandle {
    shared: Arc<Shared>,
    params: Arc<[ParamInfo]>,
    sample_rate: u32,
    notes: Arc<Mutex<rtrb::Producer<NoteMsg>>>,
    /// Id of each part's first parameter.
    part_base: [usize; PARTS],
}

impl EngineHandle {
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// The DSP parameters. The position in this slice is the parameter id.
    pub fn params(&self) -> &[ParamInfo] {
        &self.params
    }

    pub fn param_id(&self, path: &str) -> Option<usize> {
        self.params.iter().position(|p| p.path == path)
    }

    /// Set where a parameter should go. The engine glides there smoothly.
    ///
    /// Values are clamped to the parameter's range; unknown ids are ignored.
    pub fn set_target(&self, id: usize, value: f32) {
        if let (Some(p), Some(cell)) = (self.params.get(id), self.shared.targets.get(id)) {
            let v = if value.is_finite() { value } else { p.init };
            Shared::store_f32(cell, v.clamp(p.min, p.max));
        }
    }

    pub fn target(&self, id: usize) -> Option<f32> {
        self.shared.targets.get(id).map(Shared::load_f32)
    }

    pub fn levels(&self) -> Levels {
        Levels {
            peak: Shared::load_f32(&self.shared.peak),
            rms: Shared::load_f32(&self.shared.rms),
        }
    }

    /// Fade the whole output to `gain` (0..1) over `seconds`.
    ///
    /// This is the engine's transport: `fade_to(0.0, ..)` is a release to
    /// silence, `fade_to(1.0, ..)` brings the sound in. The DSP keeps running
    /// either way, so nothing about the sound itself is lost.
    pub fn fade_to(&self, gain: f32, seconds: f32) {
        let gain = if gain.is_finite() {
            gain.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let seconds = if seconds.is_finite() { seconds } else { 0.0 }.max(MIN_FADE_SECONDS);
        let current = Shared::load_f32(&self.shared.fade_current);
        let distance = (gain - current).abs().max(1e-3);
        let step = distance / (seconds * self.sample_rate as f32);
        // Speed first, then destination: the audio thread never sees a new
        // target with a stale speed for more than one chunk.
        Shared::store_f32(&self.shared.fade_step, step);
        Shared::store_f32(&self.shared.fade_target, gain);
    }

    /// Current output gain (0..1) and the gain it is fading toward.
    pub fn output_gain(&self) -> (f32, f32) {
        (
            Shared::load_f32(&self.shared.fade_current),
            Shared::load_f32(&self.shared.fade_target),
        )
    }

    /// Total frames rendered since the engine started.
    pub fn frames_rendered(&self) -> u64 {
        self.shared.frames.load(Ordering::Relaxed)
    }

    /// Queue a note event for `part`, to happen at `frame` (frames past are
    /// played at once). Returns false if the queue is full.
    pub fn note(&self, part: usize, frame: u64, event: NoteEvent) -> bool {
        if part >= PARTS {
            return false;
        }
        let mut q = self.notes.lock().unwrap_or_else(|e| e.into_inner());
        q.push(NoteMsg { part, frame, event }).is_ok()
    }

    /// Tempo for the note parts, in quarter notes per second.
    pub fn set_tempo(&self, quarters_per_second: f32) {
        if quarters_per_second.is_finite() && quarters_per_second > 0.0 {
            Shared::store_f32(&self.shared.tempo, quarters_per_second);
        }
    }

    /// Notes sounding (held or ringing out) in `part`.
    pub fn active_notes(&self, part: usize) -> usize {
        self.shared
            .voices
            .get(part)
            .map_or(0, |v| v.load(Ordering::Relaxed) as usize)
    }

    /// The id of a part's parameter, by its path inside the part (`"filter/cutoff"`).
    pub fn part_param_id(&self, part: usize, path: &str) -> Option<usize> {
        let local = synth_params::find(path)?;
        self.part_base.get(part).map(|b| b + local)
    }
}

/// Audio-thread half of the engine. Owns the DSP.
pub struct Renderer {
    dsp: Dsp,
    parts: [Part; PARTS],
    /// Number of Faust parameters; the parts' follow.
    faust_params: usize,
    notes: rtrb::Consumer<NoteMsg>,
    /// Notes received but not yet due. Preallocated; never grows.
    pending: Vec<NoteMsg>,
    frame: u64,
    note_buf: [[f32; CHUNK]; 4],
    shared: Arc<Shared>,
    /// Current (smoothed) value of each parameter, in the smoothing domain:
    /// natural log for log-scale parameters, the plain value otherwise.
    current: Box<[f32]>,
    log_domain: Box<[bool]>,
    discrete: Box<[bool]>,
    smoothing_coef: f32,
    primed: bool,
    fade: f32,
    rms_sq: f32,
    rms_coef: f32,
    peak_hold: f32,
    peak_release: f32,
    left: [f32; CHUNK],
    right: [f32; CHUNK],
}

/// Create an engine for the given sample rate.
///
/// Allocates: call this before audio starts, never on the audio thread.
pub fn new(sample_rate: u32) -> (EngineHandle, Renderer) {
    let dsp = Dsp::new(sample_rate);
    let faust_params = dsp.params().len();
    let mut all = dsp.params().to_vec();
    let mut part_base = [0; PARTS];
    for (part, base) in part_base.iter_mut().enumerate() {
        *base = all.len();
        let prefix = part_prefix(part);
        for d in synth_params::PARAMS {
            let scale = match d.kind {
                synth_params::Kind::Log => Scale::Log,
                synth_params::Kind::Linear => Scale::Linear,
                _ => Scale::Discrete,
            };
            all.push(ParamInfo::external(
                format!("{prefix}/{}", d.path),
                d.init,
                d.min,
                d.max,
                d.unit.map(str::to_string),
                scale,
            ));
        }
    }
    let params: Arc<[ParamInfo]> = all.into();
    let (producer, consumer) = rtrb::RingBuffer::new(NOTE_QUEUE);

    let targets: Box<[AtomicU32]> = params
        .iter()
        .map(|p| AtomicU32::new(p.init.to_bits()))
        .collect();
    let shared = Arc::new(Shared {
        targets,
        peak: AtomicU32::new(0f32.to_bits()),
        rms: AtomicU32::new(0f32.to_bits()),
        frames: AtomicU64::new(0),
        fade_target: AtomicU32::new(1f32.to_bits()),
        fade_step: AtomicU32::new((1.0 / (FADE_IN_SECONDS * sample_rate as f32)).to_bits()),
        fade_current: AtomicU32::new(0f32.to_bits()),
        tempo: AtomicU32::new(2f32.to_bits()),
        voices: Default::default(),
    });

    let log_domain: Box<[bool]> = params
        .iter()
        .map(|p| p.scale == Scale::Log && p.min > 0.0)
        .collect();
    let discrete: Box<[bool]> = params.iter().map(|p| p.scale == Scale::Discrete).collect();
    let current = vec![0.0; params.len()].into_boxed_slice();

    let sr = sample_rate as f32;
    let chunk_seconds = CHUNK as f32 / sr;
    let renderer = Renderer {
        dsp,
        parts: [
            Part::new(sample_rate, 0x5eed),
            Part::new(sample_rate, 0xbeef),
        ],
        faust_params,
        notes: consumer,
        pending: Vec::with_capacity(NOTE_QUEUE),
        frame: 0,
        note_buf: [[0.0; CHUNK]; 4],
        shared: shared.clone(),
        current,
        log_domain,
        discrete,
        smoothing_coef: 1.0 - (-chunk_seconds / SMOOTHING_SECONDS).exp(),
        primed: false,
        fade: 0.0,
        rms_sq: 0.0,
        rms_coef: 1.0 - (-chunk_seconds / RMS_SECONDS).exp(),
        peak_hold: 0.0,
        peak_release: (-chunk_seconds / PEAK_RELEASE_SECONDS).exp(),
        left: [0.0; CHUNK],
        right: [0.0; CHUNK],
    };
    let handle = EngineHandle {
        shared,
        params,
        sample_rate,
        notes: Arc::new(Mutex::new(producer)),
        part_base,
    };
    (handle, renderer)
}

impl Renderer {
    /// Render stereo audio into two buffers of equal length. Real-time safe.
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        assert_eq!(left.len(), right.len(), "channel buffers differ in length");
        let mut start = 0;
        while start < left.len() {
            let n = CHUNK.min(left.len() - start);
            self.render_chunk(n);
            left[start..start + n].copy_from_slice(&self.left[..n]);
            right[start..start + n].copy_from_slice(&self.right[..n]);
            start += n;
        }
    }

    /// Render into an interleaved buffer with any number of channels and any
    /// sample type. Stereo goes to the first two channels; extra channels get
    /// silence; a mono device gets the sum. Real-time safe.
    pub fn render_interleaved<T>(&mut self, out: &mut [T], channels: usize)
    where
        T: cpal::Sample + cpal::FromSample<f32>,
    {
        if channels == 0 {
            return;
        }
        let frames = out.len() / channels;
        let mut frame = 0;
        while frame < frames {
            let n = CHUNK.min(frames - frame);
            self.render_chunk(n);
            for i in 0..n {
                let base = (frame + i) * channels;
                let (l, r) = (self.left[i], self.right[i]);
                if channels == 1 {
                    out[base] = T::from_sample(0.5 * (l + r));
                } else {
                    out[base] = T::from_sample(l);
                    out[base + 1] = T::from_sample(r);
                    for c in 2..channels {
                        out[base + c] = T::EQUILIBRIUM;
                    }
                }
            }
            frame += n;
        }
    }

    fn render_chunk(&mut self, n: usize) {
        self.update_params();

        self.render_notes(n);
        let [a, b, c, d] = &self.note_buf;
        let (l, r) = (&mut self.left[..n], &mut self.right[..n]);
        self.dsp
            .compute_with([&a[..n], &b[..n], &c[..n], &d[..n]], l, r);

        let fade_target = Shared::load_f32(&self.shared.fade_target);
        let fade_step = Shared::load_f32(&self.shared.fade_step);
        let mut peak = 0.0f32;
        let mut sum_sq = 0.0f32;
        for i in 0..n {
            let g = self.fade;
            self.left[i] *= g;
            self.right[i] *= g;
            self.fade = if self.fade < fade_target {
                (self.fade + fade_step).min(fade_target)
            } else {
                (self.fade - fade_step).max(fade_target)
            };
            let (a, b) = (self.left[i].abs(), self.right[i].abs());
            peak = peak.max(a).max(b);
            sum_sq += 0.5 * (a * a + b * b);
        }
        let mean_sq = sum_sq / n.max(1) as f32;
        self.rms_sq += (mean_sq - self.rms_sq) * self.rms_coef;

        Shared::store_f32(&self.shared.fade_current, self.fade);
        // Exact digital silence (released, or not started) reads as silence
        // at once rather than as a falling peak.
        self.peak_hold = if peak == 0.0 {
            0.0
        } else {
            peak.max(self.peak_hold * self.peak_release)
        };
        Shared::store_f32(&self.shared.peak, self.peak_hold);
        Shared::store_f32(&self.shared.rms, self.rms_sq.sqrt());
        self.frame += n as u64;
        self.shared.frames.fetch_add(n as u64, Ordering::Relaxed);
    }

    /// Run the note parts for the next `n` frames into `note_buf`, playing
    /// every note due in that span on its own frame.
    fn render_notes(&mut self, n: usize) {
        while let Ok(m) = self.notes.pop() {
            if self.pending.len() < self.pending.capacity() {
                self.pending.push(m);
            }
        }
        let tempo = Shared::load_f32(&self.shared.tempo);
        for b in &mut self.note_buf {
            b[..n].fill(0.0);
        }
        let end = self.frame + n as u64;
        for (p, part) in self.parts.iter_mut().enumerate() {
            part.set_tempo(tempo);
            // Due events for this part, in arrival order, at their offsets.
            let mut due = [(0usize, NoteEvent::AllOff); 64];
            let mut count = 0;
            let mut i = 0;
            while i < self.pending.len() {
                let m = self.pending[i];
                if m.part == p && m.frame < end && count < due.len() {
                    due[count] = (m.frame.saturating_sub(self.frame) as usize, m.event);
                    count += 1;
                    // Keep the rest in order (no allocation).
                    self.pending.remove(i);
                } else {
                    i += 1;
                }
            }
            // Stable sort by offset: same-frame events keep their order.
            due[..count].sort_by_key(|&(t, _)| t);
            let [dl, dr, sl, sr] = &mut self.note_buf;
            part.render_bus(
                &due[..count],
                Bus {
                    dry_l: &mut dl[..n],
                    dry_r: &mut dr[..n],
                    send_l: &mut sl[..n],
                    send_r: &mut sr[..n],
                },
            );
            self.shared.voices[p].store(part.active_voices() as u32, Ordering::Relaxed);
        }
    }

    fn update_params(&mut self) {
        for id in 0..self.current.len() {
            let target = Shared::load_f32(&self.shared.targets[id]);
            let t = if self.log_domain[id] {
                target.ln()
            } else {
                target
            };
            let cur = &mut self.current[id];
            if self.primed && !self.discrete[id] {
                *cur += (t - *cur) * self.smoothing_coef;
            } else {
                // Nothing has sounded yet, or a count or choice (which has
                // nothing in between): take the target as is.
                *cur = t;
            }
            let value = if self.log_domain[id] { cur.exp() } else { *cur };
            if id < self.faust_params {
                self.dsp.set(id, value);
            } else {
                let local = id - self.faust_params;
                let n = synth_params::PARAMS.len();
                self.parts[local / n].set(local % n, value);
            }
        }
        self.primed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: u32 = 48_000;

    fn run(renderer: &mut Renderer, seconds: f32) -> Vec<f32> {
        let frames = (seconds * SR as f32) as usize;
        let mut l = vec![0.0; frames];
        let mut r = vec![0.0; frames];
        renderer.render(&mut l, &mut r);
        l
    }

    #[test]
    fn fades_in_from_silence() {
        let (_h, mut r) = new(SR);
        let out = run(&mut r, 0.01);
        assert!(out[0].abs() < 1e-4);
    }

    #[test]
    fn can_start_silent_then_fade_in_and_release_without_clicks() {
        let (h, mut r) = new(SR);
        h.fade_to(0.0, 0.0); // before any audio: start silent
        let silent = run(&mut r, 1.0);
        assert!(silent.iter().all(|s| s.abs() < 1e-6), "not silent");

        h.fade_to(1.0, 1.0);
        let fading_in = run(&mut r, 1.2);
        assert_eq!(h.output_gain(), (1.0, 1.0));
        let max_jump = fading_in
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .fold(0.0, f32::max);
        assert!(max_jump < 0.05, "jump of {max_jump}");

        h.fade_to(0.0, 0.5);
        let releasing = run(&mut r, 0.6);
        assert_eq!(h.output_gain(), (0.0, 0.0));
        assert!(
            releasing[releasing.len() - 1].abs() < 1e-6,
            "release did not reach silence"
        );
    }

    #[test]
    fn targets_are_clamped() {
        let (h, _r) = new(SR);
        let gain = h.param_id("master/gain").unwrap();
        h.set_target(gain, 3.0);
        assert_eq!(h.target(gain), Some(1.0));
        h.set_target(gain, f32::NAN);
        assert!(h.target(gain).unwrap().is_finite());
    }

    #[test]
    fn a_sudden_target_change_never_clicks() {
        let (h, mut r) = new(SR);
        let gain = h.param_id("master/gain").unwrap();
        run(&mut r, 2.0); // past the fade-in
        h.set_target(gain, 0.0); // an instant drop to silence
        let out = run(&mut r, 0.5);
        let max_jump = out
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .fold(0.0, f32::max);
        assert!(max_jump < 0.05, "jump of {max_jump}");
        assert!(out[out.len() - 1].abs() < 1e-3, "did not reach silence");
    }

    #[test]
    fn meters_follow_the_output() {
        let (h, mut r) = new(SR);
        run(&mut r, 2.0);
        let lv = h.levels();
        assert!(lv.rms > 0.01 && lv.peak >= lv.rms);
        assert_eq!(h.frames_rendered(), 2 * SR as u64);
    }

    #[test]
    fn the_peak_meter_holds_but_silence_reads_as_silence() {
        let (h, mut r) = new(SR);
        for _ in 0..20 {
            run(&mut r, 0.1);
            let lv = h.levels();
            assert!(lv.peak >= lv.rms, "peak {} below rms {}", lv.peak, lv.rms);
        }
        h.fade_to(0.0, 0.0);
        run(&mut r, 0.2);
        assert_eq!(h.levels().peak, 0.0);
    }

    /// An engine with the drones silent and the fade-in done.
    fn quiet(sr: u32) -> (EngineHandle, Renderer) {
        let (h, mut r) = new(sr);
        for v in ["drone_1/level", "drone_2/level"] {
            h.set_target(h.param_id(v).unwrap(), 0.0);
        }
        h.fade_to(1.0, 0.05);
        run(&mut r, 1.0);
        (h, r)
    }

    fn stereo(r: &mut Renderer, frames: usize) -> (Vec<f32>, Vec<f32>) {
        let mut a = vec![0.0; frames];
        let mut b = vec![0.0; frames];
        r.render(&mut a, &mut b);
        (a, b)
    }

    #[test]
    fn notes_start_on_their_frame() {
        let on = NoteEvent::NoteOn {
            note: 57,
            velocity: 0.8,
        };
        let (h1, mut r1) = quiet(SR);
        let (h2, mut r2) = quiet(SR);
        let base = h1.frames_rendered();
        assert!(h1.note(0, base + 1000, on));
        assert!(h2.note(0, base + 1100, on));
        let (a, _) = stereo(&mut r1, 9600);
        let (b, _) = stereo(&mut r2, 9600);
        assert!(
            a[..1000].iter().all(|s| s.abs() < 1e-7),
            "silent before the note"
        );
        assert!(
            a[1000..1100].iter().any(|s| s.abs() > 1e-4),
            "sounds from its frame"
        );
        for k in 0..8000 {
            assert!((a[1000 + k] - b[1100 + k]).abs() < 1e-6, "frame {k}");
        }
        assert_eq!(h1.active_notes(0), 1);
        assert_eq!(h1.active_notes(1), 0);
    }

    #[test]
    fn notes_sit_in_the_centre_of_the_room() {
        let (h, mut r) = quiet(SR);
        h.set_target(h.part_param_id(0, "part/reverb_send").unwrap(), 0.6);
        let base = h.frames_rendered();
        for (i, note) in [45u8, 52, 57, 60, 64, 69, 72, 76].into_iter().enumerate() {
            let at = base + i as u64 * 9600;
            h.note(
                0,
                at,
                NoteEvent::NoteOn {
                    note,
                    velocity: 0.8,
                },
            );
            h.note(0, at + 7200, NoteEvent::NoteOff { note });
        }
        let (l, rr) = stereo(&mut r, 96_000);
        let e = |x: &[f32]| x.iter().map(|s| s * s).sum::<f32>();
        let db = 10.0 * (e(&l) / e(&rr)).log10();
        assert!(db.abs() < 0.5, "notes lean {db:.2} dB");
        assert!(e(&l) > 1.0, "notes sound");
    }

    #[test]
    fn note_parameters_are_engine_parameters() {
        let (h, _r) = new(SR);
        let id = h.part_param_id(1, "filter/cutoff").unwrap();
        assert_eq!(h.params()[id].path, "note_2/filter/cutoff");
        assert_eq!(h.params()[id].scale, Scale::Log);
        let v = h.part_param_id(0, "part/voices").unwrap();
        assert_eq!(h.params()[v].scale, Scale::Discrete);
        assert!(h.part_param_id(0, "nope").is_none());
    }

    /// Real-time cost with both parts fully loaded (run with --ignored --nocapture).
    #[test]
    #[ignore]
    fn cost_with_both_parts_full() {
        let (h, mut r) = new(SR);
        for part in 0..PARTS {
            for (p, v) in [
                ("osc1/unison", 7.0),
                ("osc2/unison", 7.0),
                ("osc2/level", 0.6),
                ("fx/chorus_mix", 0.5),
                ("fx/delay_mix", 0.4),
                ("shape/drive", 0.4),
                ("filter/slope", 24.0),
            ] {
                h.set_target(h.part_param_id(part, p).unwrap(), v);
            }
            for k in 0..4 {
                h.note(
                    part,
                    0,
                    NoteEvent::NoteOn {
                        note: 48 + 7 * k as u8,
                        velocity: 1.0,
                    },
                );
            }
        }
        run(&mut r, 0.1);
        let t = std::time::Instant::now();
        run(&mut r, 10.0);
        let share = t.elapsed().as_secs_f64() / 10.0;
        println!(
            "engine with 8 heavy notes: {:.1}% of one core",
            share * 100.0
        );
    }

    #[test]
    fn interleaved_handles_any_channel_count() {
        let (_h, mut r) = new(SR);
        let mut mono = vec![0.0f32; 100];
        r.render_interleaved(&mut mono, 1);
        let mut quad = vec![1.0f32; 400];
        r.render_interleaved(&mut quad, 4);
        assert!(quad.chunks(4).all(|f| f[2] == 0.0 && f[3] == 0.0));
        let mut ints = vec![0i16; 200];
        r.render_interleaved(&mut ints, 2);
    }
}
