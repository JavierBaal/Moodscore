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

pub mod audio;

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use moodscore_dsp::Dsp;
pub use moodscore_dsp::{ParamInfo, Scale};

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
    /// Peak of the most recent chunk, linear (0..1).
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
}

/// Audio-thread half of the engine. Owns the DSP.
pub struct Renderer {
    dsp: Dsp,
    shared: Arc<Shared>,
    /// Current (smoothed) value of each parameter, in the smoothing domain:
    /// natural log for log-scale parameters, the plain value otherwise.
    current: Box<[f32]>,
    log_domain: Box<[bool]>,
    smoothing_coef: f32,
    primed: bool,
    fade: f32,
    rms_sq: f32,
    rms_coef: f32,
    left: [f32; CHUNK],
    right: [f32; CHUNK],
}

/// Create an engine for the given sample rate.
///
/// Allocates: call this before audio starts, never on the audio thread.
pub fn new(sample_rate: u32) -> (EngineHandle, Renderer) {
    let dsp = Dsp::new(sample_rate);
    let params: Arc<[ParamInfo]> = dsp.params().to_vec().into();

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
    });

    let log_domain: Box<[bool]> = params
        .iter()
        .map(|p| p.scale == Scale::Log && p.min > 0.0)
        .collect();
    let current = vec![0.0; params.len()].into_boxed_slice();

    let sr = sample_rate as f32;
    let chunk_seconds = CHUNK as f32 / sr;
    let renderer = Renderer {
        dsp,
        shared: shared.clone(),
        current,
        log_domain,
        smoothing_coef: 1.0 - (-chunk_seconds / SMOOTHING_SECONDS).exp(),
        primed: false,
        fade: 0.0,
        rms_sq: 0.0,
        rms_coef: 1.0 - (-chunk_seconds / RMS_SECONDS).exp(),
        left: [0.0; CHUNK],
        right: [0.0; CHUNK],
    };
    let handle = EngineHandle {
        shared,
        params,
        sample_rate,
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

        let (l, r) = (&mut self.left[..n], &mut self.right[..n]);
        self.dsp.compute(l, r);

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
        Shared::store_f32(&self.shared.peak, peak);
        Shared::store_f32(&self.shared.rms, self.rms_sq.sqrt());
        self.shared.frames.fetch_add(n as u64, Ordering::Relaxed);
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
            if self.primed {
                *cur += (t - *cur) * self.smoothing_coef;
            } else {
                // Nothing has sounded yet: start exactly at the targets.
                *cur = t;
            }
            let value = if self.log_domain[id] { cur.exp() } else { *cur };
            self.dsp.set(id, value);
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
