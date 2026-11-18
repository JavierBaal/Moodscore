//! Moodscore audio engine.
//!
//! Owns the audio callback: drains parameter frames, smooths every parameter
//! toward its target, runs the DSP and feeds output analysis to `listen`.
//!
//! Not implemented yet (Phase 1). See `docs/ARCHITECTURE.md`.
