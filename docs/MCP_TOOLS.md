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

`over` defaults to 3 seconds, or `over_bars` (✅) fades in over so many bars. `start_in` (Phase 2 ✅) delays the fade; `at_bar` (✅) starts it on the downbeat of that bar instead (worked out with the tempo map at the time of the call).

### `release` ✅

Fade the whole piece out to silence. A spoken "stop", "quiet" or "enough" should map here. The instrument keeps its state; `start` brings it back.

```json
{ "over": 12, "start_in": 40 }
```

`over` defaults to 8 seconds, or `over_bars` (✅) fades out over so many bars. `start_in` (Phase 2 ✅) delays the fade, or `at_bar` (✅) starts it on a given bar's downbeat (an ending timed to the music, no seconds to count), so a gesture can peak at full volume before the sound dies away (an unstable outro, say). A waiting fade shows up in `output.scheduled_fade`; any new `start` or `release` replaces it.

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
- **Chords** are Roman numerals in the key. Case is quality: upper case major, lower case minor, so `V` in a minor key is the borrowed major V. `b`/`#` before the numeral alters the root, counted from the major scale as borrowed chords are usually written: `bVII` is a whole tone below the tonic in any mode (in D dorian it is C, the same as `VII`), `bVI` and `bIII` the flat sixth and third, `#iv°` a raised fourth. `°` (or `o`, `dim`) diminished, `+` (or `aug`) augmented, `ø` half-diminished. Colour, in any order after the numeral: `7` (the mode's seventh), `maj7`, `sus2`, `sus4`, `add9` (the mode's ninth). A slash and a note name put that note in the bass: `I/C#`, `IVmaj7/A` (it wins over a `pedal` for that chord; `V/V` is refused: write the secondary dominant itself). Notes are spelled properly by letter (`iv` in C major is F A♭ C).
- **Durations**: each chord gives exactly one of `seconds` (0.5–600), `beats` (0.5–1024, in the metre's beat unit) or `bars` (0.25–256). Beats and bars follow the tempo clock (see `tempo`), ramps included; a tempo change mid-chord stretches or shrinks what is left of it. Every change lands exactly when it is due, so a looping progression never drifts off the beat.
- **`start_on`**: `"now"` (default), `"beat"` (next beat) or `"bar"` (next downbeat). Or **`at_bar`**: the downbeat of that bar (the state shows the current one in `tempo.bar`; a bar that has passed is refused). Give `harmony`, each `play_notes` and a `release` the same `at_bar` and they line up exactly, however far apart the calls are. While a plan waits, the state shows `harmony.starts_in`.
- **`loop`** (default true). Without it, the last chord holds.
- **`pedal`**: a note held in the bass under every chord (`A`, or `E1`…`E2` with an octave). Without it the bass voice (`drone_1`) follows each chord's root.
- **`glide`**: seconds each voice takes to reach its next note (default 3); **`curve`** shapes it. With short chords, lower it so each chord arrives before the next.
- Voicing: `drone_1` is the bass (E1–E2), `drone_2`…`drone_6` share the chord (E2–A4). Every chord tone is covered when there are enough voices (dropping the fifth first when there aren't); extra voices double the root or fifth, or whatever moves least.
- Harmony moves pitches only, never levels: the chord sounds through whichever voices are up (bring them in with the `density` macro, or `gesture` on `drone_N/level`).
- **Ownership**: harmony takes every voice pitch nobody has set by hand. A pitch retuned with `gesture` leaves the harmony and stays put (`get_state` shows it as `manual` with `held_from: ["harmony"]`) until a `harmony` call with `"reclaim": true`.
- A new call replaces the plan and starts now from its first chord, voice-leading from wherever the voices are, so changing key or mode mid-piece is just another call. A plan that waits for a later beat (`start_on`, `at_bar`) lets the current one walk on until then (the state shows it under `harmony.then`), and the sequencer already sees the chords coming. `{ "stop": true }` stops walking: voices stay on their notes and become free.
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
| Filter | `filter/type` (`low_pass`, `band_pass`, `high_pass`: clean state-variable; `ladder`: a four-pole Moog-style low-pass that saturates and thickens, self-oscillating at full resonance, keeping most of its bass), `filter/slope` (12, 24; not for the ladder), `filter/cutoff` (Hz), `filter/resonance`, `filter/keytrack`, `filter/env_amount` (octaves), `filter/drive` (ladder only) |
| Envelopes | `amp_env/…`, `filter_env/…`, `mod_env/…`: `attack`, `decay`, `sustain`, `release` (seconds; sustain 0–1); `amp_env/velocity` |
| Playing | `part/voices` (1–4; 1 is monophonic), `part/legato` (monophonic: overlapping notes slide without restarting the envelopes; a detached note starts on its own pitch), `part/glide` (s; with legato, only between overlapping notes), `part/expression` (a free control for the agent), `part/analog` (0 exact; 0.3 alive; 1 an old synth: each note slightly out of tune and wandering, its cutoff and envelope times a little different) |
| LFOs | `lfo1/…`, `lfo2/…`: `shape` (`sine`, `triangle`, `saw`, `square`, `random`), `rate` (Hz), `sync`, `division` (`1/32` … `4`, with `t` triplets and `d` dotted), `delay`, `fade` (s), `retrigger` |
| Modulation | `mod1/…` … `mod8/…`: `source` (`mod_env`, `filter_env`, `amp_env`, `lfo1`, `lfo2`, `velocity`, `note`, `random`, `expression`), `dest` (`pitch/bend`, `voice/gain`, `voice/pan`, the oscillators' `level`, `shape`, `pw`, `fine`, `detune`, `sync`, `fm/amount`, `sub/level`, `noise/level`, `shape/fold`, `shape/drive`, `filter/cutoff`, `filter/resonance`, `filter/drive`, `lfo1/rate`, `lfo2/rate`), `amount` (−1 … 1, a fraction of the target's range; of its octaves for frequencies; `pitch/bend` spans ±12 semitones, so 0.01 ≈ a quarter tone) |
| Effects | `fx/distortion`, `fx/ensemble` (string-machine ensemble: three swaying copies per side, very wide; below ~140 Hz the sound stays mono), `fx/chorus_mix`, `fx/chorus_rate`, `fx/chorus_depth`, `fx/delay_mix`, `fx/delay_division`, `fx/delay_feedback`, `fx/delay_width`, `fx/delay_tone` (Hz) |
| Mix | `part/level`, `part/pan`, `part/reverb_send` |

### `play_notes` ✅

Play a phrase: to audition a patch, or to place a motif or a line.

```json
{ "part": 2, "phrase": "B3:0.5 D4 G4:1 | C#4:0.5 E4 A4:1 | A3+C#4+E4:2@0.6 r:1 F#5:1.5~ G5:0.5", "repeat": 4, "at_bar": 9 }
```

- **`phrase`**, the short form: tokens separated by spaces. `A3:0.5` is a note and its length in beats (C4 is middle C; `1/2` and `3/4` also work); a missing length repeats the previous one (1 at the start). `A3+C#4+E4:2` is a chord; `r:1` (or `-:1`) a rest; `@0.6` sets that step's velocity; a trailing `~` holds the note into the next one (gate 1.05), so a legato patch slides. `|` bar lines are for the eye and are ignored. **`velocity`** (default 0.8) and **`gate`** (fraction held, default 0.9) set the defaults.
- **`steps`**, the long form, one object per step: `notes` (a note, a chord as `"A3 C4 E4"`, a MIDI number, or `"rest"`), `beats`, `velocity`, `gate`.
- Up to 1024 steps, `repeat` up to 64. Follows the tempo map (ramps included); each note starts on its exact sample. `start_on`: `now`, `beat`, `bar`, or `at_bar`.
- Notes wait in the conductor's agenda and reach the audio thread about 200 ms before they are due. A new phrase adds to what is waiting; `{ "stop": true }` cancels what is waiting on that part and releases what sounds. The state shows, per part, `waiting` and `phrase_ends_in`.
- For anything that repeats, use `sequence`: one call, and it follows the harmony.

### `sequence` ✅

Play a sequence on a note part: an arpeggio over the chord sounding, or a motif (an ostinato), stepping with the tempo and following the harmony on its own, until stopped or for some `bars`.

```json
{ "part": 1, "arp": "up_down", "range": ["F4", "C#5"], "rate": "1/8t", "legato": true, "at_bar": 17 }
{ "part": 2, "motif": "1 - - 5~ 8 - 5 <5", "around": "A#1", "rate": "1/8", "bars": 8 }
{ "part": 1, "motif": "F4 r C5 A#4 r F4 C5 r", "rate": "1/16" }
```

- **`arp`**: `up`, `down`, `up_down`, `down_up` (the top and bottom notes are not repeated), `random` (never the same note twice in a row), `chord` (all its notes on every step). Each chord of `harmony` is spread inside **`range`** `[low, high]` (default `["C4", "C5"]`), so the notes two chords share stay put; the arpeggio starts again from its first note on each new chord. **`notes`** (`"A3 C4 E4"`) arpeggiates those instead, with no harmony needed. **`rhythm`** is one character per step, repeating: `x` play, `X` accent (velocity + `accent`, default 0.15), `.` rest, `-` hold the previous note (`"x.xX .x-x"`; default `"x"`).
- **`motif`**: one token per step, up to 64. **Degrees follow the harmony**: `1` is the chord's root, `3` `5` `7` its own third, fifth and seventh (a borrowed major V brings its leading tone), `2` `4` `6` come from the key; `b` and `#` alter from the major scale (`b7` is always a minor seventh), `8` is the octave, `<5` an octave down, `>3` an octave up. Each chord's root goes as close to **`around`** as it can (default `"C4"`; `"A#1"` for a bass line). **Note names stay put** while the harmony moves (`F4`, `A#3`; upper case, so `b3` is a degree and `B3` a note). `r` rest, `-` hold, `~` after a note ties it into the next, `@0.9` its velocity, `+` several at once (`1+5`). A motif carries its own rhythm, so `rhythm` is for arpeggios.
- **`rate`**: the step, `1/16` (default), `1/8`, `1/8t` (triplet), `1/4d` (dotted)…; it follows the tempo map, ramps included. **`gate`** (0.05–1, default 0.8), **`velocity`** (default 0.8), **`swing`** (0 to just under 0.5: every second step lands that fraction of a step late; 0.33 is a shuffle), **`legato`** (tie every note into the next; with a legato patch such as `glide_arp` or `moog_bass` they slide).
- **When**: on the next bar by default, or `start_on` (`now`, `beat`, `bar`) or `at_bar`. A new sequence on the same part takes over there: the old one lets go of its notes on that downbeat, so a section change is one call per part, on the beat. **`bars`** limits how long it plays (default: until stopped or replaced). `{ "stop": true }` ends it on the next bar (`start_on: "now"` for at once); `play_notes` with `stop` ends it at once too.
- A chord counts for a step if it lands up to 60 ms after it, so a plan started a hair late still changes on the downbeat; start `harmony` with `start_on: "bar"` or `at_bar` so the drones change there too.
- Without harmony, an arpeggio over the chord or a motif in degrees is refused (with a hint); after `harmony` stops, they keep to the last chord.
- Returns the brief report plus `sequence` (`plays`, in words, `starts_in`, `ends_in`). The state shows each part's `sequences`: the one playing and the next if one waits for its bar.
- How it runs: the conductor works each sequence out about half a second ahead, step by step, asking the director which chord sounds when each step lands, and puts the notes in the same agenda as `play_notes`, fixed to their exact samples on one clock-to-frame line shared by every part. Hear it first with `audition` and `sequence`.

### `list_presets` ✅ / `save_preset` ✅

`list_presets` gives each preset's name, description and whether it is built in. `save_preset` (`part`, `name`, `description`, `overwrite`) saves a part's sound; it can't shadow a built-in name. Saved presets are JSON files in `MOODSCORE_PRESETS`, or the platform's data folder (`%APPDATA%\Moodscore\presets` on Windows, `~/Library/Application Support/Moodscore/presets` on macOS, `~/.local/share/moodscore/presets` elsewhere). Built-in presets live in `presets/` in the repository: `pluck`, `lead`, `moog_bass` (deep mono bass, sliding), `swell_horn` (horns that open as they fade), `glide_arp` (soft legato arpeggio) and `deep_pad` (wide, slow pad). The last four were tuned with `audition` against the targets of the same family, and pass them both dry and in the room.

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
- **`at_bar`** or **`start_on`** (`now`, `beat`, `bar`): the change happens on that downbeat instead of now (a ritardando from bar 33). It waits on the tempo map, so the harmony, the sequences and the phrases worked out before it already follow it; the state lists it under `tempo.changes_ahead` (bar, seconds, bpm). Up to eight can wait. A change now leaves the waiting ones on their beats, except a new `downbeat`, which starts the count over and drops them.
- Changes never move the beat: the position carries on unbroken, and a glide starts from the tempo of that moment (even in the middle of another glide).
- The state reports `tempo`: `bpm`, `meter`, `bar` and `beat` now, `seconds_per_bar`, and while a glide is under way `ramping_to` and `ramp_seconds_remaining`.
- The tempo map is analytic: the beat at any instant is computed exactly from the last change, not accumulated, so it is exact at any sample. The director reads it for the harmony, and the sequencer for every step.

### `sections` ✅

Lay out the piece as sections the engine plays on its own, one after another. One call is the whole structure: no waiting between calls, and the music carries on if the conversation stops.

```json
{
  "at_bar": 1,
  "sections": [
    { "name": "intro", "bars": 8, "actions": [
        { "harmony": { "key": "Bb", "progression": [{ "chord": "i", "bars": 2 }, { "chord": "VI", "bars": 2 }] } },
        { "patch": { "part": 1, "preset": "deep_pad" } },
        { "sequence": { "part": 1, "motif": "1+3+5+8 - - - - - - -", "rate": "1/4" } },
        { "morph": { "darkness": 0.8, "over_bars": 8 } } ] },
    { "name": "build", "bars": 8, "actions": [
        { "patch": { "part": 2, "preset": "moog_bass" } },
        { "sequence": { "part": 2, "motif": "1 - - 1~ 5 - 8 <5", "around": "A#1", "rate": "1/8" } },
        { "gesture": { "param": "drone_5/level", "to": 0.4, "over_bars": 4 }, "after_bars": 4 } ] },
    { "name": "outro", "bars": 4, "actions": [
        { "tempo": { "bpm": 100, "over_bars": 4 } },
        { "release": { "over_bars": 4 } } ] }
  ]
}
```

- **Sections** follow each other: each lasts **`bars`** (counted in the metre in force when it begins, so a tempo change stretches it in seconds, never in bars) or **`seconds`** (for music without a pulse). The first begins on the next bar (or `start_on`, `at_bar`).
- **Actions** are the tools you call directly, with the same fields: `harmony`, `tempo`, `morph`, `gesture`, `set_pace`, `patch`, `sequence`, `play_notes`, `start`, `release`. Each happens at the section's start, or **`after_bars`** (`after_seconds`) into it. The section says when, so `start_on`, `at_bar` and `start_in` are refused inside. **`over_bars`** in `morph`, `gesture`, `start`, `release` and `tempo` makes a move last bars instead of seconds.
- **Atomic**: the whole call is checked before anything is scheduled (chords, presets, motifs, parameters, ranges, an action past its section's end). If one action is wrong, nothing happens and the error names the section and the action.
- **On the beat**: a tempo change, a harmony, a sequence or a phrase is set up about 0.6 s ahead of its moment and lands exactly on it (the tempo change waits on the tempo map, the harmony waits behind the plan walking, the notes are fixed to their samples); a `patch` lands just before its bar so the first note has the new sound; morphs, gestures and fades fire on the moment. Actions due together run tempo first, then pace, harmony, patches, moves, fades, notes, stops.
- **`loop`** repeats the list. Without it the last section's sound stays (unless it releases), and the list is done when the last section ends.
- **Changing course**: a new call replaces what has not happened yet; what is sounding plays on until a section changes it. To change the rest of a piece, send the sections still to come with `at_bar`. `{ "stop": true }` ends the list.
- The state shows `sections`: the `section` playing, its `bar` of `of_bars`, the `next` and `next_in` seconds (or `ends_in`), `loop`, every section's name, and `errors` (an action that failed when its moment came: a sequence refused for lack of harmony, say).

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

Macros: `darkness` ✅, `instability` ✅, `density` ✅, `distance` ✅; `tension`, `motion`, `urgency` (Phase 2).

| Macro | 0 | 1 | Default | Drives |
|---|---|---|---|---|
| `darkness` | open and airy | closed, low and murky | 0.4 | filter cutoff and resonance, sub, reverb |
| `instability` | steady and pure | wavering, beating, seasick | 0.3 | pitch drift and detune of every drone voice |
| `density` | the root alone | all six voices | 0.15 (root and a quiet fifth, the drone at rest) | `drone_2` … `drone_6` levels, each over its own stretch of the macro (low to high: 2 by 0.15, 3 by 0.35, 4 by 0.55, 5 by 0.75, 6 by 0.95), and a slight trim of the master as it fills |
| `distance` | near and dry | far and washed | 0.4 (the room as it always was) | the room: `space/size` (3 to 15.5 s), `space/damping` (9 to 3.3 kHz), `space/predelay` (35 ms near, none from 0.4), `space/wet` (half near), `space/direct` (down to 0.4 far) and `space/air` (the highs of the direct sound, far), and adds to `space/reverb_mix` |

Omitted macros keep moving (or stay) as they were. At their defaults `density` and `distance` leave the sound exactly as it was before they existed. `distance` acts on the shared room, so the note parts move with the drones without any of their own parameters changing (their `part/reverb_send` stays theirs). A drone level set by hand with `gesture` leaves `density`'s hands like any hand-set parameter.

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

`over_bars` (✅) instead of `over` makes it last so many bars on the tempo map; the same goes for `morph`.

#### Voices ✅ Phase 2

Six drone voices, `drone_1` … `drone_6`, each with `pitch` (Hz), `level`, `detune` (cents) and `drift`; `drone_1` also has `sub`. At rest `drone_1` (A1) and a quiet `drone_2` (E2) sound; `drone_3` … `drone_6` wait at level 0 on A2, C3, E3, A3 (an open A minor chord). Bring voices in and out with the `density` macro; for one particular voice, `gesture` on `drone_N/level` (it then leaves `density`'s hands); retune with `drone_N/pitch` (or let `harmony` do it).

The room (shared by drones and notes): `space/reverb_mix` (the drones' share of reverb), `space/size` (tail, s), `space/damping` (Hz above which the tail darkens), `space/predelay` (ms before the room answers), `space/wet` (how loud it answers), `space/direct` (the direct sound of everything) and `space/air` (how much of its top the air takes, 0 none). `distance` drives them all.

### `voice`

Bring a voice in or out, or retune it.

```json
{ "voice": "drone_2", "action": "enter", "pitch": "A1", "over": 15 }
```

### `hold`

Freeze everything at its current position (cancel pending gestures without jumping).

## Awareness

### `get_state` ✅ Phase 1

The current pace, any start or release waiting to begin, every macro and parameter with its current value, target, owner (`macro:<name>` or `manual`) and seconds of movement remaining, plus output level (RMS and peak, dBFS). The note parts appear under `notes` (preset, modified, notes sounding, phrase notes waiting and when the last one plays, and the `sequences` playing or waiting); their parameters are reported by `patch`, not here. Voice states and changes the human made by hand arrive in later phases.

### `audition` ✅

Claude's ears for its own sounds: render a note part's current sound playing a phrase, offline (nothing sounds), and measure it.

```json
{ "part": 1, "phrase": "A#1:4~ F2:4", "target": "deep_moog_bass" }
```

- `phrase` in the short form of `play_notes` (default: the target's phrase, else `"A3:2"`); `bpm` (default: the target's, else the current tempo); `room: true` adds the shared reverb and master, with the drones silent.
- Or `sequence` (the same fields as the `sequence` tool) and `bars` (default 2, up to 16): plays it from the next bar over the harmony and tempo in force, or over chords and a tempo of its own (`harmony`, with the harmony tool's fields, and `bpm`), which only the audition hears: nothing that is playing changes. `played` says the bars, bpm, the chords under it and how many notes.
- Returns `summary` (the sound in words), `level`, `spectrum` (centroid, energy per band, tilt in dB/octave, noisiness), `stereo` (correlation and balance below 150 Hz, 150 Hz–3 kHz and 3–12 kHz), `over_time` (brightness and level in eight stretches), and per note: attack, change in level over the note, brightness at 10/50/90 %, harmonics 2–8 against the fundamental (single notes), tuning, wobble and glide time. `metrics` has the headline numbers, flat.
- With `target`, `comparison` lists each criterion: value, target, tolerance, and "on target" or how far above or below.

### `listen` ✅

The same measurements on the last `seconds` (1–30, default 5) of the live output: the whole mix as it sounds. Without notes to go by, the pitch heard is tracked instead (`pitch`: held notes, glides, wobble, the longest note's harmonics; `pitch_range` sets where to look). `target` compares.

### `analyze_file` ✅

The same measurements on a reference WAV the person put in the references folder (`MOODSCORE_REFERENCES`; else `referencias/` at the root of the checkout the binary was built in, kept out of git; else the platform's data folder). `file` is a name inside that folder, never a path. `from`/`to` pick a stretch (at most 60 s), measured as one event; `band` ([low, high] Hz) singles out an instrument in a mix; `pitch_range` says where to look for its pitch. In a full mix, harmonic levels and glides are approximate. The same analysis is on the command line: `moodscore analyze FILE.wav [--from S] [--to S] [--band LO HI] [--pitch LO HI] [--target FILE]`.

### Targets

Target sounds are data in `presets/targets/` (numbers only, no audio): a name, a description, the `phrase` and `bpm` to audition with, and `criteria` (`metric: [value, tolerance]`). `list_presets` lists them. The first four were measured from reference tracks: `deep_moog_bass`, `swell_horn`, `glide_arp`, `deep_pad`.

### `list_parameters`

Every raw parameter with its range, unit and a one-line description, so the agent can discover the instrument instead of memorizing it.

## Safety

### `panic`

Immediate but click-free fade to silence (about 1 s). For emergencies, not for musical endings.

---

## Why not expose only raw parameters?

An agent can drive forty knobs, but it describes music badly that way, and every call becomes long and fragile. Macros let the agent speak in the same terms as the user ("darker", "closer", "more unstable"), while `gesture` keeps precise control available when it's needed.
