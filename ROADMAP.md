# Roadmap

Each phase ends with something you can hear. Order may change; scope will.

## Phase 0: Skeleton

- Cargo workspace and crate layout
- Faust → Rust build pipeline working on Windows, macOS and Linux
- CI: build and test on all three platforms

## Phase 1: Proof of concept

The milestone that decides whether the idea works:

> Tell the agent *"get darker, slowly"*, and it sounds right.

- One drone voice, one virtual-analog filter, one reverb
- Engine with parameter smoothing and real-time-safe audio thread
- Gesture director (trajectories, curves, interruption without jumps)
- MCP server with three tools: `morph` (one macro: `darkness`), `gesture`, `get_state`

## Phase 2: The instrument

- Six drone voices and two melodic voices
- All six mood macros with data-driven mappings
- Scenes: `set_scene`, `save_scene`, `list_scenes`, factory scene library
- `voice`, `hold`, `release`, `panic`

## Phase 3: Listening

- Output analysis and the `listen` tool
- Human overrides reported through `get_state`
- Agent guide: a prompt or skill teaching agents how to perform with Moodscore

## Phase 4: Watching it play

- Minimal GUI: macros, active gestures and voices moving in real time
- Manual control of macros, with the agent aware of your changes

## Phase 5: Scoring to picture

- Load a video, detect scene cuts, extract keyframes
- Agent drafts a cue sheet from the picture
- Timeline playback in sync with the video; refine by conversation
- Render to audio file

## Later

- VST3 / CLAP plugin builds
- Live DSP editing via Faust JIT
- MIDI and OSC in/out
