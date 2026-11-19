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

## Harmony (planned, Phase 2)

### `harmony`

Set a harmonic plan. The engine walks the progression, voice-leading smoothly between chords.

```json
{
  "key": "E",
  "mode": "aeolian",
  "progression": [
    { "chord": "i",   "bars": 4 },
    { "chord": "VI",  "bars": 4 },
    { "chord": "III", "bars": 4 },
    { "chord": "VII", "bars": 4 }
  ],
  "loop": true,
  "pedal": "E1",
  "transition": "glide"
}
```

Durations may also be given in seconds when no tempo is running.

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

## Time and pulse (planned, Phase 2)

### `tempo`

Set or glide the tempo. Glides are gestures: an accelerando is `tempo` with `over`.

```json
{ "bpm": 90, "over": 120, "curve": "ease_in" }
```

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

The current pace, any start or release waiting to begin, every macro and parameter with its current value, target, owner (`macro:<name>` or `manual`) and seconds of movement remaining, plus output level (RMS and peak, dBFS). Voice states and changes the human made by hand arrive in later phases.

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
