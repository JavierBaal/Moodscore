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

**Agreed order from here (19 Nov 2026):**

1. ✅ Tempo clock, sample-exact, with tempo gestures; harmony durations in bars and beats. The map is analytic and allocation-free, ready to be read from the audio thread by the step sequencer.
2. ✅ The synth engine for Claude: the core of the two note voices (see *Synth engine* below), with the first presets (pluck, lead).
3. ✅ Audition: Claude's ears (render, analyse, describe): `audition`, `listen`, `analyze_file`, targets measured from reference tracks. ✅ The sound quality pass in the same block: ladder filter, analog drift, ensemble width, and four presets tuned against the targets (`moog_bass`, `swell_horn`, `glide_arp`, `deep_pad`); pluck and lead moved to the ladder, ensemble and drift.
4. Step sequencer: arpeggios and ostinatos played by the note voices.
5. Chained sections: a cue list the engine runs on its own.
6. `density` and `distance` macros.

Then `tension`, the pulse voice and `urgency`, scenes, `voice`, `hold`, `panic`.

Sound:
- [x] Six drone voices (A minor chord at rest, two sounding), with energy-based normalisation
- [x] Two note voices (melodic), driven by the synth engine below (`note_1`, `note_2`; `patch`, `play_notes`)
- All six mood macros with data-driven mappings
  - [x] `instability` (pitch drift and detune)
  - [x] Macros can share a parameter (`"mode": "add"`) and target many with `*` patterns
  - [ ] `tension`: dissonance and suspensions come with `harmony`; until then it would only be resonance and beating

Harmony:
- [x] The six drone voices form a chord; the agent sets a harmonic plan (key, mode, Roman-numeral progression with durations in seconds, loop, pedal) and the engine walks it, voices gliding to the nearest chord tone
- [x] Mode as colour: all seven modes; borrowed and altered chords; 7, maj7, sus2, sus4, add9
- [ ] Crossfading between chords (needs spare voices)
- [ ] `tension` adds dissonance (suspensions, added ninths, clusters) on top of the harmony

Synth engine for Claude (agreed with Fran on 19 Nov): a Serum/Vital-style synth designed for an agent to shape sounds with, as the engine of the two note voices (and, later, perhaps the drones):
Done (core, `moodscore-synth`): everything in this list except true wavetables and some presets (soft pluck, glass bell, aggressive lead). Effects are per part (distortion, chorus, ensemble, tempo-synced echo); the reverb is the shared room. Added with the sound quality pass: a Moog-style ladder filter with drive, analog drift (`part/analog`) and a stereo ensemble that keeps the lows mono. Built-in presets: pluck, lead, moog_bass, swell_horn, glide_arp, deep_pad.

- Oscillators: shape morphing (sine, triangle, saw, square, pulse width), FM between oscillators, sync, wavefolding, unison with detune and spread, sub and noise. Table-free to respect the no-locks rule; true wavetables later, written in Rust without locks.
- Filters: low-pass, band-pass, high-pass, and a Moog-style ladder with drive.
- Envelopes and LFOs, and a modulation matrix (any source to any parameter, a fixed number of slots).
- Effects: distortion, chorus, delay, reverb.
- Polyphony where it matters: a pluck needs its tail to ring under the next note (about 4 notes per voice); a lead is monophonic with portamento and delayed vibrato.
- Presets as data, with names and descriptions, editable without recompiling: pluck, lead, moog_bass, swell_horn, glide_arp, deep_pad; still to come: soft pluck, glass bell, aggressive lead.
- Tools to set a patch by name, save and load presets, and describe the current patch.
- Audition, the part that makes it a synth *for Claude*: render a note or a phrase offline (not through the speakers), analyse it and return a description (brightness, attack, decay, harmonic content, noise, stereo width, pitch stability, loudness), so Claude can iterate by numbers while the person judges by ear.
- Boundary: the note voices play motifs, ostinatos and arpeggios that repeat and follow the harmony. Long melodies composed note by note stay in Four Hands Studio; Phase 3's generative melody will build on these voices.

Arpeggios and ostinatos (planned with Fran on 19 Nov; need the tempo clock and the note voices):
- Step sequencer, 1–16 steps, sample-accurate in the audio thread
- Arpeggios over the chord that is sounding (up, down, up-down, random; octaves; rate), following the harmony
- Ostinatos: a short motif that either follows the harmony (transposed with each chord) or stays fixed while the harmony moves under it

Pace and time:
- [x] `set_pace`: a standing default for how long transitions take and their shape ("everything slow today", "short and direct"), reported in the state so every client honours it
- [x] Tempo clock with tempo gestures: accelerando and ritardando glide like everything else, never jump (`tempo`; chords in bars and beats follow it)
- Tempo-synced modulation: LFOs, filter pulsing, delays

Pulse (cinematic rhythm, not a drum machine):
- One pulse voice: synthetic percussive generator, from a low heartbeat to a metallic hit, long tails welcome
- Step sequencer, 1–16 steps: triggers pulses, or modulates the drone's pitch and filter (in the spirit of the Solar 42F's sequencer)
- The agent sets patterns; the engine plays them sample-accurately
- `urgency` macro: pulse density, tempo and tension together ("time is running out")

More macros, from the same live tests (each section needed many single gestures):
- `density`: how many voices sound, 0..1, bringing voices in and out in a musical order
- `distance`: near and dry to far and washed (reverb and its colour)

Session:
- Chained sections: a cue list (harmony, macros, voices and fades, each with when it enters, in seconds or bars) that the engine runs on its own. In the live tests of 19 Nov the agent chained sections by sleeping in a shell between calls: fragile (if the conversation stops, the piece freezes in its current section; Claude Desktop has no shell) and it nearly lost a section to an already-scheduled release.
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

- [x] Output analysis and the `listen` tool (brought forward into Phase 2 with `audition`)
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
