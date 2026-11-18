//! Types and traits the Faust Rust backend expects to find in scope.
//!
//! These mirror the interface the Faust compiler generates code against.
//! They are written for Moodscore rather than copied from Faust's
//! architecture files.

#![allow(dead_code)]

pub type F32 = f32;
pub type F64 = f64;
pub type FaustFloat = F32;

/// Index of a parameter inside the generated DSP.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ParamIndex(pub i32);

/// Placeholder for Faust soundfiles (unused by Moodscore for now).
pub struct Soundfile<'a, T> {
    pub f_buffers: &'a &'a T,
    pub f_length: &'a i32,
    pub f_sr: &'a i32,
    pub f_offset: &'a i32,
    pub f_channels: i32,
}

/// The interface implemented by Faust-generated DSP code.
pub trait FaustDsp {
    type T;

    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]);
}

/// Receives DSP-level metadata (`declare name "..."` and so on).
pub trait Meta {
    fn declare(&mut self, key: &str, value: &str);
}

/// Receives the description of the DSP's controls.
pub trait UI<T> {
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}
