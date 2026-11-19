//! Moodscore's note synth.
//!
//! A subtractive and FM synth designed for an agent to shape sounds with:
//! two morphing oscillators with unison and hard sync, FM, sub and noise,
//! a 2× oversampled wavefolder and saturator, a state-variable filter,
//! three envelopes, two LFOs and an eight-slot modulation matrix, played
//! by a [`Part`] with up to four voices (or one, with legato and glide). Every parameter is a number in a flat table
//! ([`params::PARAMS`]), so a patch is plain data.
//!
//! Like the DSP crate, it knows nothing about moods, gestures or MCP.
//! Nothing here allocates once a voice exists, so it can run on the audio
//! thread.

pub mod dsp;
pub mod lfo;
pub mod osc;
pub mod params;
pub mod part;
pub mod voice;

pub use params::{Kind, ParamDef, Patch, P, PARAMS};
pub use part::{Event, Part, MAX_VOICES};
pub use voice::{Ctx, Voice};
