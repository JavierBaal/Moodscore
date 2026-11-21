//! Moodscore's sound engine DSP.
//!
//! The synthesis itself is written in Faust (`dsp/moodscore.dsp`) and
//! compiled to Rust at build time. This crate wraps the generated code in a
//! small, safe API: discover parameters, set them, render audio.
//!
//! This crate knows nothing about moods, gestures or MCP.

mod faust;

#[allow(
    clippy::all,
    unused_parens,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    unused_mut,
    unused_imports,
    static_mut_refs
)]
mod generated {
    use super::faust::*;
    include!(concat!(env!("OUT_DIR"), "/moodscore_dsp.rs"));
}

use faust::{FaustDsp, Meta, ParamIndex, UI};
use generated::MoodscoreDsp;

/// Number of outputs: stereo, and the drones as they enter the room (mono,
/// for metering only).
pub const NUM_OUTPUTS: usize = 3;

/// Frames [`Dsp::compute_with`] renders at a time when it throws the
/// metering output away.
const SCRATCH: usize = 256;

/// Number of audio inputs: the note voices' dry sound (left, right) and
/// their reverb send (left, right).
pub const NUM_INPUTS: usize = 4;

/// How a parameter's range is meant to be traversed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Scale {
    Linear,
    Log,
    /// Whole steps (a count or a choice): set, never glided.
    Discrete,
}

/// Description of one DSP parameter, as declared in the Faust source.
#[derive(Clone, Debug)]
pub struct ParamInfo {
    /// Stable path, e.g. `"filter/cutoff"`. This is what higher layers use.
    pub path: String,
    pub init: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub unit: Option<String>,
    pub scale: Scale,
    index: ParamIndex,
}

impl ParamInfo {
    /// A parameter that lives outside the Faust DSP (the note synth's).
    /// [`Dsp::set`] ignores it; the engine routes it.
    pub fn external(
        path: String,
        init: f32,
        min: f32,
        max: f32,
        unit: Option<String>,
        scale: Scale,
    ) -> Self {
        Self {
            path,
            init,
            min,
            max,
            step: if scale == Scale::Discrete { 1.0 } else { 0.0 },
            unit,
            scale,
            index: ParamIndex(-1),
        }
    }
}

/// The Moodscore DSP: parameters in, stereo audio out.
pub struct Dsp {
    inner: Box<MoodscoreDsp>,
    params: Vec<ParamInfo>,
    sample_rate: u32,
}

impl Dsp {
    /// Create and initialise the DSP. Allocates: never call this on the audio thread.
    pub fn new(sample_rate: u32) -> Self {
        let mut inner = allocate();
        FaustDsp::init(&mut *inner, sample_rate as i32);

        let mut collector = ParamCollector::default();
        FaustDsp::build_user_interface(&*inner, &mut collector);

        Self {
            inner,
            params: collector.params,
            sample_rate,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// All parameters, in a stable order. Use the position in this slice as the
    /// parameter id for [`Dsp::set`] and [`Dsp::get`].
    pub fn params(&self) -> &[ParamInfo] {
        &self.params
    }

    /// Look up a parameter id by its path (e.g. `"filter/cutoff"`).
    pub fn param_id(&self, path: &str) -> Option<usize> {
        self.params.iter().position(|p| p.path == path)
    }

    /// Set a parameter, clamped to its declared range. Real-time safe.
    ///
    /// Unknown ids are ignored. This sets the value immediately: smoothing over
    /// time is the engine's job.
    pub fn set(&mut self, id: usize, value: f32) {
        if let Some(p) = self.params.get(id) {
            let v = if value.is_finite() { value } else { p.init };
            FaustDsp::set_param(&mut *self.inner, p.index, v.clamp(p.min, p.max));
        }
    }

    /// Current value of a parameter.
    pub fn get(&self, id: usize) -> Option<f32> {
        self.params
            .get(id)
            .and_then(|p| FaustDsp::get_param(&*self.inner, p.index))
    }

    /// Render one block of stereo audio with no notes. Real-time safe.
    ///
    /// Both buffers must have the same length; that length is the block size.
    pub fn compute(&mut self, left: &mut [f32], right: &mut [f32]) {
        const SILENCE: [f32; 256] = [0.0; 256];
        let n = left.len();
        let mut start = 0;
        while start < n {
            let m = (n - start).min(SILENCE.len());
            let z = &SILENCE[..m];
            let (l, r) = (&mut left[start..start + m], &mut right[start..start + m]);
            self.compute_with([z, z, z, z], l, r);
            start += m;
        }
    }

    /// Render one block of stereo audio, mixing in the note voices: their
    /// dry sound (left, right) and their reverb send (left, right), each as
    /// long as the output. Real-time safe.
    pub fn compute_with(
        &mut self,
        notes: [&[f32]; NUM_INPUTS],
        left: &mut [f32],
        right: &mut [f32],
    ) {
        assert_eq!(left.len(), right.len(), "channel buffers differ in length");
        let mut drones = [0.0f32; SCRATCH];
        let n = left.len();
        let mut start = 0;
        while start < n {
            let m = (n - start).min(SCRATCH);
            let [a, b, c, d] = notes;
            let r = start..start + m;
            self.compute_metered(
                [&a[r.clone()], &b[r.clone()], &c[r.clone()], &d[r.clone()]],
                &mut left[start..start + m],
                &mut right[start..start + m],
                &mut drones[..m],
            );
            start += m;
        }
    }

    /// As [`compute_with`](Self::compute_with), and also write the drones
    /// as they enter the room (mono, made room in for the notes) to
    /// `drones`, for metering. Real-time safe.
    pub fn compute_metered(
        &mut self,
        notes: [&[f32]; NUM_INPUTS],
        left: &mut [f32],
        right: &mut [f32],
        drones: &mut [f32],
    ) {
        assert_eq!(left.len(), right.len(), "channel buffers differ in length");
        assert_eq!(
            left.len(),
            drones.len(),
            "metering buffer differs in length"
        );
        assert!(
            notes.iter().all(|b| b.len() == left.len()),
            "input length differs"
        );
        let frames = left.len() as i32;
        let mut outputs: [&mut [f32]; NUM_OUTPUTS] = [left, right, drones];
        FaustDsp::compute(&mut *self.inner, frames, &notes, &mut outputs);
    }
}

/// Allocate the DSP state directly on the heap.
///
/// The generated struct holds large delay lines (hundreds of kilobytes), so
/// building it on the stack first, as `Box::new(MoodscoreDsp::new())` would in
/// a debug build, can overflow small thread stacks.
fn allocate() -> Box<MoodscoreDsp> {
    // SAFETY: the Faust-generated struct is `#[repr(C)]` and contains only
    // `i32` and `f32` fields and fixed-size arrays of them, for which the
    // all-zero bit pattern is a valid value. `MoodscoreDsp::new()` itself
    // initialises every field to zero; `init()` is called right after.
    unsafe {
        let layout = std::alloc::Layout::new::<MoodscoreDsp>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut MoodscoreDsp;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Box::from_raw(ptr)
    }
}

/// Collects parameter descriptions from the generated UI description.
#[derive(Default)]
struct ParamCollector {
    params: Vec<ParamInfo>,
    pending_unit: Vec<(i32, String)>,
    pending_log: Vec<i32>,
}

impl ParamCollector {
    fn add(&mut self, label: &str, index: ParamIndex, init: f32, min: f32, max: f32, step: f32) {
        let unit = self
            .pending_unit
            .iter()
            .find(|(i, _)| *i == index.0)
            .map(|(_, u)| u.clone());
        let scale = if self.pending_log.contains(&index.0) {
            Scale::Log
        } else {
            Scale::Linear
        };
        self.params.push(ParamInfo {
            path: label.to_string(),
            init,
            min,
            max,
            step,
            unit,
            scale,
            index,
        });
    }
}

impl UI<f32> for ParamCollector {
    fn open_tab_box(&mut self, _label: &str) {}
    fn open_horizontal_box(&mut self, _label: &str) {}
    fn open_vertical_box(&mut self, _label: &str) {}
    fn close_box(&mut self) {}

    fn add_button(&mut self, label: &str, param: ParamIndex) {
        self.add(label, param, 0.0, 0.0, 1.0, 1.0);
    }
    fn add_check_button(&mut self, label: &str, param: ParamIndex) {
        self.add(label, param, 0.0, 0.0, 1.0, 1.0);
    }
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.add(label, param, init, min, max, step);
    }
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.add(label, param, init, min, max, step);
    }
    fn add_num_entry(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        self.add(label, param, init, min, max, step);
    }

    fn add_horizontal_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}
    fn add_vertical_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}

    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str) {
        let Some(param) = param else { return };
        match key {
            "unit" => self.pending_unit.push((param.0, value.to_string())),
            "scale" if value == "log" => self.pending_log.push(param.0),
            _ => {}
        }
    }
}

/// Faust's own metadata for the DSP (`name`, `version`, …).
pub fn metadata() -> Vec<(String, String)> {
    struct Collect(Vec<(String, String)>);
    impl Meta for Collect {
        fn declare(&mut self, key: &str, value: &str) {
            self.0.push((key.to_string(), value.to_string()));
        }
    }
    let dsp = allocate();
    let mut c = Collect(Vec::new());
    FaustDsp::metadata(&*dsp, &mut c);
    c.0
}
