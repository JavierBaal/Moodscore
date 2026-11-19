# MCP tools (draft)

> Tools marked **✅ Phase 1** are implemented; the rest are the plan. Names and shapes may still change. Feedback welcome in issues.

These are the tools an agent uses to play Moodscore. They are designed around one rule: **the agent describes where the sound should go and how long it should take to get there; the engine does the moving.**

## Conventions

- Macros and most parameters are normalized to `0.0–1.0`.
- Durations are in seconds (`over`). `0` means "as fast as is musically safe" (a short smoothing time, never a hard jump).
- Curves: `linear`, `ease_in`, `ease_out`, `ease_in_out`, `exponential`, `logarithmic`.
- `over` defaults to 10 seconds; `curve` defaults to `ease_in_out`.
- Every call returns the resulting state, so the agent never has to guess.
- Invalid calls (unknown names, values out of range) return a tool error explaining what was wrong; nothing changes.

## Transport

Moodscore starts silent when an MCP client launches it. Nothing sounds until the agent calls `start`.

### `start` ✅

Fade in from silence. After a `release`, brings the sound back exactly as it was.

```json
{ "over": 3 }
```

`over` defaults to 3 seconds.

### `release` ✅

Fade the whole piece out to silence. A spoken "stop", "quiet" or "enough" should map here. The instrument keeps its state; `start` brings it back.

```json
{ "over": 12 }
```

`over` defaults to 8 seconds.

### `set_pace` (planned, Phase 2)

Set the default transition time and shape for every tool call that doesn't specify its own.

```json
{ "over": 30, "curve": "ease_in_out" }
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

Move one or more mood macros toward new values. Parameters a macro drives come back under its control, gliding from wherever they are over the same duration.

```json
{
  "tension": 0.7,
  "darkness": 0.6,
  "over": 30,
  "curve": "ease_in"
}
```

Macros: `darkness` (Phase 1); `tension`, `density`, `motion`, `distance`, `instability`, `urgency` (Phase 2). Omitted macros keep moving (or stay) as they were.

### `set_scene`

Morph the whole engine toward a saved scene.

```json
{ "name": "hangar", "over": 20 }
```

### `save_scene` / `list_scenes`

Snapshot the current state under a name, and list available scenes with short descriptions.

## Fine control

### `gesture` ✅ Phase 1

Move a single raw parameter along a trajectory. For when a macro is too coarse. The parameter leaves macro control until the next `morph` of its macro.

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

## Awareness

### `get_state` ✅ Phase 1

Every macro and parameter with its current value, target, owner (`macro:<name>` or `manual`) and seconds of movement remaining, plus output level (RMS and peak, dBFS). Voice states and changes the human made by hand arrive in later phases.

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
