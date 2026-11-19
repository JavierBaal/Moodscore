# MCP tools (draft)

> Tools marked **✅** are implemented; the rest are the plan. Names and shapes may still change. Feedback welcome in issues.

These are the tools an agent uses to play Moodscore. They are designed around one rule: **the agent describes where the sound should go and how long it should take to get there; the engine does the moving.**

## Conventions

- Macros and most parameters are normalized to `0.0–1.0`.
- Durations are in seconds (`over`). `0` means "as fast as is musically safe" (a short smoothing time, never a hard jump).
- Curves: `linear`, `ease_in`, `ease_out`, `ease_in_out`, `exponential`, `logarithmic`. In the schemas a curve is a plain string, validated by the server and lenient with case, spaces and hyphens (`"Ease-In"` works); unknown names are rejected with the list of valid ones.
- When a call omits `over` or `curve`, it uses the current pace: 10 seconds and `ease_in_out` unless `set_pace` changed it. Transport fades (`start`, `release`) have their own defaults.
- Every call returns the resulting state, so the agent never has to guess. Action tools return a brief report: output, pace, macros (each standing for the parameters it drives) and, under `moving`, only the hand-set parameters still moving. `get_state` returns everything.
- Invalid calls (unknown names, values out of range) return a tool error explaining what was wrong; nothing changes.

## Transport

Moodscore starts silent when an MCP client launches it. Nothing sounds until the agent calls `start`.

### `start` ✅

Fade in from silence. After a `release`, brings the sound back exactly as it was.

```json
{ "over": 3, "start_in": 0 }
```

`over` defaults to 3 seconds. `start_in` (Phase 2 ✅) delays the fade.

### `release` ✅

Fade the whole piece out to silence. A spoken "stop", "quiet" or "enough" should map here. The instrument keeps its state; `start` brings it back.

```json
{ "over": 12, "start_in": 40 }
```

`over` defaults to 8 seconds. `start_in` (Phase 2 ✅) delays the fade, so a gesture can peak at full volume before the sound dies away (an unstable outro, say). A waiting fade shows up in `output.scheduled_fade`; any new `start` or `release` replaces it.

### `set_pace` ✅ Phase 2

Set the standing default for how long moves take and their shape, for every `morph` and `gesture` that doesn't give its own `over` or `curve`. For when the person sets a general feel: *"everything slow today"*, *"short and direct"*.

```json
{ "over": 30, "curve": "ease_in_out" }
```

Both fields are optional, but at least one is required; an omitted field keeps its current value. `over: 0` means "as fast as is musically safe". Moves already under way keep their timing. The current pace is reported by every tool under `pace`.

## Harmony ✅ Phase 2

### `harmony` ✅

Set a harmonic plan. The six drone voices walk the progression on their own, each voice gliding to the nearest note of the next chord (smooth voice leading, like a string section).

```json
{
  "key": "A",
  "mode": "aeolian",
  "progression": [
    { "chord": "i",   "seconds": 20 },
    { "chord": "VI",  "seconds": 20 },
    { "chord": "III", "seconds": 20 },
    { "chord": "VII", "seconds": 20 }
  ],
  "loop": true,
  "pedal": "A",
  "glide": 4
}
```

- **`key`**: tonic (`A`, `F#`, `Bb`). **`mode`**: `ionian`, `dorian`, `phrygian`, `lydian`, `mixolydian`, `aeolian` (default), `locrian`, or `major` / `minor`. Mode is colour: aeolian dark, dorian bittersweet, phrygian ominous, lydian luminous, mixolydian open and heroic, locrian unstable.
- **Chords** are Roman numerals in the key. Case is quality: upper case major, lower case minor, so `V` in a minor key is the borrowed major V. `b`/`#` before the numeral alters the root (`bVI`, `bVII`). `°` (or `o`, `dim`) diminished, `+` (or `aug`) augmented, `ø` half-diminished. Colour, in any order after the numeral: `7` (the mode's seventh), `maj7`, `sus2`, `sus4`, `add9` (the mode's ninth). Notes are spelled properly by letter (`iv` in C major is F A♭ C).
- **Durations**: each chord gives exactly one of `seconds` (0.5–600), `beats` (0.5–1024, in the metre's beat unit) or `bars` (0.25–256). Beats and bars follow the tempo clock (see `tempo`), ramps included; a tempo change mid-chord stretches or shrinks what is left of it. Every change lands exactly when it is due, so a looping progression never drifts off the beat.
- **`start_on`**: `"now"` (default), `"beat"` (next beat) or `"bar"` (next downbeat). While a plan waits, the state shows `harmony.starts_in`.
- **`loop`** (default true). Without it, the last chord holds.
- **`pedal`**: a note held in the bass under every chord (`A`, or `E1`…`E2` with an octave). Without it the bass voice (`drone_1`) follows each chord's root.
- **`glide`**: seconds each voice takes to reach its next note (default 3); **`curve`** shapes it. With short chords, lower it so each chord arrives before the next.
- Voicing: `drone_1` is the bass (E1–E2), `drone_2`…`drone_6` share the chord (E2–A4). Every chord tone is covered when there are enough voices (dropping the fifth first when there aren't); extra voices double the root or fifth, or whatever moves least.
- Harmony moves pitches only, never levels: the chord sounds through whichever voices are up (bring them in with `gesture` on `drone_N/level`).
- **Ownership**: harmony takes every voice pitch nobody has set by hand. A pitch retuned with `gesture` leaves the harmony and stays put (`get_state` shows it as `manual` with `held_from: ["harmony"]`) until a `harmony` call with `"reclaim": true`.
- A new call replaces the plan and starts now from its first chord, voice-leading from wherever the voices are, so changing key or mode mid-piece is just another call. `{ "stop": true }` stops walking: voices stay on their notes and become free.
- The state reports `harmony`: key, the chord sounding (`symbol`, `name`, `notes`), the `next` chord, `seconds_to_next`, `position` (e.g. `"2/4"`), `loop` and `pedal`.

## Note parts ✅ Phase 2

Two note parts, `note_1` and `note_2`, play notes over the drones: plucks, leads, motifs. Each is the same synth (`moodscore-synth`); the patch decides what it is. Notes share the drones' room (reverb), mono bass, master gain and soft clip.

### `patch` ✅

Shape a part's sound: load a preset, change parameters, or with neither just describe it.

```json
{ "part": 1, "preset": "pluck", "set": { "filter/cutoff": 900, "mod2/source": "lfo1", "mod2/dest": "voice/pan", "mod2/amount": 0.4 }, "over": 0 }
```

- `part`: `1` or `2` (or `"note_1"`, `"note_2"`). Default 1.
- `preset`: loaded first; every parameter is set, so earlier tweaks don't linger.
- `set`: path → value. Numbers, or choices by name. Continuous parameters glide over `over` seconds (default 0); counts and choices switch.
- Returns the part's `description` (in words), every parameter that differs from the default (`params`, the same shape as a preset file), the `preset` loaded, whether it was `modified` since, and how many notes are `sounding`.

The parameters (inside a part; as engine parameters they are `note_1/<path>`, so `gesture` can move them too, e.g. `note_1/part/expression`):

| Group | Parameters |
|---|---|
| Oscillators | `osc1/…`, `osc2/…`: `level`, `shape` (0 sine, 1 triangle, 2 saw, 3 square; morphs in between), `pw`, `octave`, `semi`, `fine` (cents), `unison` (1–7), `detune` (cents), `spread` (stereo width), `sync` (1 = off; up to 8), `random_phase` |
| Colour | `fm/amount` (osc2 modulates osc1), `sub/level`, `sub/octave`, `noise/level`, `shape/fold`, `shape/drive` |
| Filter | `filter/type` (`low_pass`, `band_pass`, `high_pass`), `filter/slope` (12, 24), `filter/cutoff` (Hz), `filter/resonance`, `filter/keytrack`, `filter/env_amount` (octaves) |
| Envelopes | `amp_env/…`, `filter_env/…`, `mod_env/…`: `attack`, `decay`, `sustain`, `release` (seconds; sustain 0–1); `amp_env/velocity` |
| Playing | `part/voices` (1–4; 1 is monophonic), `part/legato`, `part/glide` (s), `part/expression` (a free control for the agent) |
| LFOs | `lfo1/…`, `lfo2/…`: `shape` (`sine`, `triangle`, `saw`, `square`, `random`), `rate` (Hz), `sync`, `division` (`1/32` … `4`, with `t` triplets and `d` dotted), `delay`, `fade` (s), `retrigger` |
| Modulation | `mod1/…` … `mod8/…`: `source` (`mod_env`, `filter_env`, `amp_env`, `lfo1`, `lfo2`, `velocity`, `note`, `random`, `expression`), `dest` (`pitch/bend`, `voice/gain`, `voice/pan`, the oscillators' `level`, `shape`, `pw`, `fine`, `detune`, `sync`, `fm/amount`, `sub/level`, `noise/level`, `shape/fold`, `shape/drive`, `filter/cutoff`, `filter/resonance`, `lfo1/rate`, `lfo2/rate`), `amount` (−1 … 1, a fraction of the target's range; of its octaves for frequencies; `pitch/bend` spans ±12 semitones, so 0.01 ≈ a quarter tone) |
| Effects | `fx/distortion`, `fx/chorus_mix`, `fx/chorus_rate`, `fx/chorus_depth`, `fx/delay_mix`, `fx/delay_division`, `fx/delay_feedback`, `fx/delay_width`, `fx/delay_tone` (Hz) |
| Mix | `part/level`, `part/pan`, `part/reverb_send` |

### `play_notes` ✅

Play a short phrase: to audition a patch, or to place a motif.

```json
{ "part": 1, "steps": [ { "notes": "A3", "beats": 0.5 }, { "notes": "C4 E4", "beats": 0.5, "velocity": 0.6 }, { "notes": "rest", "beats": 1 } ], "repeat": 4, "start_on": "bar" }
```

- `notes`: a note (`A3`; C4 is middle C), a chord (`"A3 C4 E4"`), a MIDI number, or `"rest"`. `beats` in the current metre (default 1), `velocity` 0–1 (default 0.8), `gate` the fraction held (default 0.9; above 1 overlaps the next note, so a legato patch slides).
- Follows the tempo map (ramps included) and starts on each note's exact sample. `start_on`: `now`, `beat`, `bar`. `repeat` up to 16. `{ "stop": true }` releases the part.
- Until the step sequencer arrives, this is how notes are played. The sequencer (arpeggios and ostinatos that follow the harmony) runs in the engine.

### `list_presets` ✅ / `save_preset` ✅

`list_presets` gives each preset's name, description and whether it is built in. `save_preset` (`part`, `name`, `description`, `overwrite`) saves a part's sound; it can't shadow a built-in name. Saved presets are JSON files in `MOODSCORE_PRESETS`, or the platform's data folder (`%APPDATA%\Moodscore\presets` on Windows, `~/Library/Application Support/Moodscore/presets` on macOS, `~/.local/share/moodscore/presets` elsewhere). Built-in presets live in `presets/` in the repository.

## Melody (planned, Phase 3)

### `melody`

Either rules for generative melody, or a written motif.

```json
{ "mode": "generative", "register": "mid", "density": 0.2, "contour": "falling", "phrase_length": [2, 5] }
```

```json
{ "mode": "motif", "notes": ["B3", "G3", "F#3", "E3"], "durations": [1, 1, 0.5, 2.5], "save_as": "theme_a" }
```

### `recall_motif`

Bring back a saved motif, optionally transformed.

```json
{ "name": "theme_a", "transform": { "tempo": 0.5, "mode": "minor", "transpose": -12 } }
```

## Time and pulse (Phase 2)

### `tempo` ✅

The musical clock: tempo and metre. It runs from the moment the engine starts, at 120 BPM in 4/4.

```json
{ "bpm": 138, "meter": "4/4", "downbeat": true }
```

```json
{ "bpm": 70, "over_bars": 4, "curve": "ease_out" }
```

- **`bpm`** (20–300) counts the metre's beat: the quarter note in 4/4 or 3/4, the eighth in 6/8. Omit it to keep the current speed (a metre change then keeps the quarter note's speed).
- **`meter`**: `"4/4"`, `"3/4"`, `"6/8"`, `"5/4"`… A new metre starts a new bar at once.
- **`over`** (seconds) or **`over_bars`** (bars of the new metre): glide to the new tempo instead of switching, an accelerando or ritardando. Default: switch at once. **`curve`** shapes the glide (default `linear`). A ramp in bars covers exactly that many bars, whatever the curve.
- **`downbeat`**: make this moment bar 1, beat 1. Otherwise the bar count carries on.
- Changes never move the beat: the position carries on unbroken, and a glide starts from the tempo of that moment (even in the middle of another glide).
- The state reports `tempo`: `bpm`, `meter`, `bar` and `beat` now, `seconds_per_bar`, and while a glide is under way `ramping_to` and `ramp_seconds_remaining`.
- The tempo map is analytic: the beat at any instant is computed exactly from the last change, not accumulated, so it is exact at any sample. Today the director reads it (harmony); the step sequencer will read the same map in the audio thread.

### `pulse`

Set the pulse voice's step pattern and sound. Patterns loop in time with the tempo; the engine plays them sample-accurately.

```json
{
  "steps": [1, 0, 0, 0, 1, 0, 0, 0],
  "sound": "heartbeat",
  "level": 0.6,
  "over": 8
}
```

A step value is the hit's velocity (`0` = rest). The sequencer can also target the drone instead of triggering hits (e.g. stepping the filter).

## Mood

### `morph` ✅ Phase 1

Move one or more mood macros toward new values. Parameters set by hand with `gesture` stay where they were put; pass `"reclaim": true` to bring them back under the macros, gliding from wherever they are over the same duration.

```json
{
  "tension": 0.7,
  "darkness": 0.6,
  "over": 30,
  "curve": "ease_in"
}
```

Macros: `darkness` ✅, `instability` ✅; `tension`, `density`, `motion`, `distance`, `urgency` (Phase 2).

| Macro | 0 | 1 | Drives |
|---|---|---|---|
| `darkness` | open and airy | closed, low and murky | filter cutoff and resonance, sub, reverb |
| `instability` | steady and pure | wavering, beating, seasick | pitch drift and detune of every drone voice | Omitted macros keep moving (or stay) as they were.

### `set_scene`

Morph the whole engine toward a saved scene.

```json
{ "name": "hangar", "over": 20 }
```

### `save_scene` / `list_scenes`

Snapshot the current state under a name, and list available scenes with short descriptions.

## Fine control

### `gesture` ✅ Phase 1

Move a single raw parameter along a trajectory. For when a macro is too coarse. The parameter leaves macro control and stays where it is put: later morphs leave it alone (`get_state` lists it as `manual`, with `held_from` naming the macros) until a `morph` with `"reclaim": true`.

```json
{
  "param": "drone_4/level",
  "to": 0.4,
  "over": 60,
  "curve": "ease_in_out",
  "start_in": 10
}
```

#### Voices ✅ Phase 2

Six drone voices, `drone_1` … `drone_6`, each with `pitch` (Hz), `level`, `detune` (cents) and `drift`; `drone_1` also has `sub`. At rest `drone_1` (A1) and a quiet `drone_2` (E2) sound; `drone_3` … `drone_6` wait at level 0 on A2, C3, E3, A3 (an open A minor chord). Until `harmony` and `voice` exist, bring voices in and out and retune them with `gesture` on `drone_N/level` and `drone_N/pitch`.

### `voice`

Bring a voice in or out, or retune it.

```json
{ "voice": "drone_2", "action": "enter", "pitch": "A1", "over": 15 }
```

### `hold`

Freeze everything at its current position (cancel pending gestures without jumping).

## Awareness

### `get_state` ✅ Phase 1

The current pace, any start or release waiting to begin, every macro and parameter with its current value, target, owner (`macro:<name>` or `manual`) and seconds of movement remaining, plus output level (RMS and peak, dBFS). The note parts appear under `notes` (preset, modified, notes sounding); their parameters are reported by `patch`, not here. Voice states and changes the human made by hand arrive in later phases.

### `listen`

A short perceptual report of the actual output over a window.

```json
{ "window": 5 }
```

Returns, for example:

```json
{
  "loudness_lufs": -21.4,
  "brightness": 0.32,
  "density": 0.55,
  "movement": 0.18,
  "dominant_pitch": "D2",
  "notes": "low-mid heavy, slow beating between drones 1 and 3"
}
```

### `list_parameters`

Every raw parameter with its range, unit and a one-line description, so the agent can discover the instrument instead of memorizing it.

## Safety

### `panic`

Immediate but click-free fade to silence (about 1 s). For emergencies, not for musical endings.

---

## Why not expose only raw parameters?

An agent can drive forty knobs, but it describes music badly that way, and every call becomes long and fragile. Macros let the agent speak in the same terms as the user ("darker", "closer", "more unstable"), while `gesture` keeps precise control available when it's needed.
