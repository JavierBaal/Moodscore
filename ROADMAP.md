# Roadmap

Each phase ends with something you can hear. Order may change; scope will.

## Phase 0: Skeleton ✅

- [x] Cargo workspace and crate layout
- [x] Faust → Rust build pipeline, with pre-generated fallback when Faust is not installed
- [x] First drone voice (detuned saws + sub, analog drift, ladder filter, stereo reverb)
- [x] `moodscore render` to check the sound end to end
- [x] CI: fmt, clippy, tests and a render smoke test on Windows, macOS and Linux

## Phase 1: Proof of concept ✅ (validated live with Claude Desktop)

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
- [x] Six drone voices (A minor chord at rest, two sounding), with energy-based normalisation
- [ ] Two melodic voices (after harmony and the tempo clock: they need notes to play)
- All six mood macros with data-driven mappings
  - [x] `instability` (pitch drift and detune)
  - [x] Macros can share a parameter (`"mode": "add"`) and target many with `*` patterns
  - [ ] `tension`: dissonance and suspensions come with `harmony`; until then it would only be resonance and beating

Harmony:
- [x] The six drone voices form a chord; the agent sets a harmonic plan (key, mode, Roman-numeral progression with durations in seconds, loop, pedal) and the engine walks it, voices gliding to the nearest chord tone
- [x] Mode as colour: all seven modes; borrowed and altered chords; 7, maj7, sus2, sus4, add9
- [ ] Crossfading between chords (needs spare voices)
- [ ] `tension` adds dissonance (suspensions, added ninths, clusters) on top of the harmony

Arpeggios and ostinatos (planned with Fran on 19 Nov; needs the tempo clock and a note voice):
- One note voice with an envelope (from plucked to pad): the first of the two melodic voices, brought forward
- Step sequencer, 1–16 steps, sample-accurate in the audio thread
- Arpeggios over the chord that is sounding (up, down, up-down, random; octaves; rate), following the harmony
- Ostinatos: a short motif that either follows the harmony (transposed with each chord) or stays fixed while the harmony moves under it

Pace and time:
- [x] `set_pace`: a standing default for how long transitions take and their shape ("everything slow today", "short and direct"), reported in the state so every client honours it
- Tempo clock with tempo gestures: accelerando and ritardando glide like everything else, never jump
- Tempo-synced modulation: LFOs, filter pulsing, delays

Pulse (cinematic rhythm, not a drum machine):
- One pulse voice: synthetic percussive generator, from a low heartbeat to a metallic hit, long tails welcome
- Step sequencer, 1–16 steps: triggers pulses, or modulates the drone's pitch and filter (in the spirit of the Solar 42F's sequencer)
- The agent sets patterns; the engine plays them sample-accurately
- `urgency` macro: pulse density, tempo and tension together ("time is running out")

Session:
- [x] `start_in` for `start` and `release` (delayed fades, e.g. an outro that peaks before it dies)
- [x] Live-play polish: curves as plain strings in the tool schemas, hand-set parameters stay put across morphs (`reclaim` to hand them back), brief reports grouped by macro
- Scenes: `set_scene`, `save_scene`, `list_scenes`, factory scene library
- `voice`, `hold`, `panic`

Out of scope: full drum kits and groove programming. That's composition, and belongs in a DAW (or Four Hands Studio); Moodscore may share a clock with it later.

## Phase 3: Melody

Needs the tempo clock and the two melodic voices from Phase 2.

- Generative melody within rules the agent sets: scale, register, density, contour, silence ("short falling phrases, lots of space"). The agent directs; the engine improvises
- Written motifs: the agent writes a short theme (4–8 notes) and the engine plays it in time
- Leitmotifs: save a theme and bring it back transformed (slower, in minor, another register) when a character or idea returns

Out of scope: fully composed songs with melody, harmony and form written note by note. That's Four Hands Studio territory; Moodscore does harmonic fields and motifs.

## Phase 4: Listening

- Output analysis and the `listen` tool
- Human overrides reported through `get_state`
- Agent guide: a prompt or skill teaching agents how to perform with Moodscore

## Phase 5: Watching it play

- Minimal GUI: macros, active gestures and voices moving in real time
- Manual control of macros, with the agent aware of your changes

## Phase 6: Scoring to picture

- Load a video, detect scene cuts, extract keyframes
- Agent drafts a cue sheet from the picture
- Timeline playback in sync with the video; refine by conversation
- Render to audio file

## Later

- VST3 / CLAP plugin builds
- Live DSP editing via Faust JIT
- MIDI and OSC in/out
- Clock sync with other tools (Ableton Link, MIDI clock)
