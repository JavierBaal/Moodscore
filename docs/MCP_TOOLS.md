# MCP tools (draft)

> Draft API. Names and shapes will change during the prototype. Feedback welcome in issues.

These are the tools an agent uses to play Moodscore. They are designed around one rule: **the agent describes where the sound should go and how long it should take to get there; the engine does the moving.**

## Conventions

- Macros and most parameters are normalized to `0.0–1.0`.
- Durations are in seconds (`over`). `0` means "as fast as is musically safe" (a short smoothing time, never a hard jump).
- Curves: `linear`, `ease_in`, `ease_out`, `ease_in_out`, `exponential`, `logarithmic`.
- Every call returns the resulting state, so the agent never has to guess.

## Mood

### `morph`

Move one or more mood macros toward new values.

```json
{
  "tension": 0.7,
  "darkness": 0.6,
  "over": 30,
  "curve": "ease_in"
}
```

Macros: `tension`, `darkness`, `density`, `motion`, `distance`, `instability`. Omitted macros keep moving (or stay) as they were.

### `set_scene`

Morph the whole engine toward a saved scene.

```json
{ "name": "hangar", "over": 20 }
```

### `save_scene` / `list_scenes`

Snapshot the current state under a name, and list available scenes with short descriptions.

## Fine control

### `gesture`

Move a single raw parameter along a trajectory. For when a macro is too coarse.

```json
{
  "param": "drone_4.level",
  "to": 0.4,
  "over": 60,
  "curve": "ease_in_out",
  "start_in": 10
}
```

### `voice`

Bring a voice in or out, or retune it.

```json
{ "voice": "drone_2", "action": "enter", "pitch": "A1", "over": 15 }
```

### `hold`

Freeze everything at its current position (cancel pending gestures without jumping).

### `release`

Fade the whole piece out over a given time. A spoken "stop" from the user should map here, not to silence.

```json
{ "over": 12 }
```

## Awareness

### `get_state`

Current macros, active gestures (with time remaining), voice states, and any changes the human made by hand since the last call.

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
