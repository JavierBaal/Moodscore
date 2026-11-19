# Roadmap

Each phase ends with something you can hear. Order may change; scope will.

## Phase 0: Skeleton ✅

- [x] Cargo workspace and crate layout
- [x] Faust → Rust build pipeline, with pre-generated fallback when Faust is not installed
- [x] First drone voice (detuned saws + sub, analog drift, ladder filter, stereo reverb)
- [x] `moodscore render` to check the sound end to end
- [x] CI: fmt, clippy, tests and a render smoke test on Windows, macOS and Linux

## Phase 1: Proof of concept ✅ (built; waiting on listening tests)

The milestone that decides whether the idea works:

> Tell the agent *"get darker, slowly"*, and it sounds right.

- [x] One drone voice, one virtual-analog filter, one reverb
- [x] Engine with parameter smoothing, fade-in, level meters and a real-time-safe audio thread (cpal)
- [x] Gesture director: trajectories, six curves, delayed starts, interruption without jumps, macro/manual ownership
- [x] `darkness` macro mapped from data (`macros/default.json`)
- [x] MCP server over stdio with `start`, `release`, `morph`, `gesture`, `get_state`; silent until `start`
- [x] `play` (live demo), `render` (offline, same path as live), `serve --no-audio` and an MCP smoke test in CI

## Phase 2: The instrument

Sound:
- Six drone voices and two melodic voices
- All six mood macros with data-driven mappings, starting with `instability` and `tension`

Pace and time:
- `set_pace`: a standing default for how long transitions take and their shape ("everything slow today", "short and direct"), reported in the state so every client honours it
- Tempo clock with tempo gestures: accelerando and ritardando glide like everything else, never jump
- Tempo-synced modulation: LFOs, filter pulsing, delays

Pulse (cinematic rhythm, not a drum machine):
- One pulse voice: synthetic percussive generator, from a low heartbeat to a metallic hit, long tails welcome
- Step sequencer, 1–16 steps: triggers pulses, or modulates the drone's pitch and filter (in the spirit of the Solar 42F's sequencer)
- The agent sets patterns; the engine plays them sample-accurately
- `urgency` macro: pulse density, tempo and tension together ("time is running out")

Session:
- Scenes: `set_scene`, `save_scene`, `list_scenes`, factory scene library
- `voice`, `hold`, `panic`

Out of scope: full drum kits and groove programming. That's composition, and belongs in a DAW (or Four Hands Studio); Moodscore may share a clock with it later.

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
- Clock sync with other tools (Ableton Link, MIDI clock)
