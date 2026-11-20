# Moodscore

**A drone engine for live, mood-driven film scoring over MCP.**

Tell it how the scene should feel. It moves the sound there.

> **Status: Phase 2 in progress.** You can play it live from Claude (or any MCP client): six drone voices that walk chord progressions in any mode with smooth voice leading, a musical clock (tempo, metre, accelerando and ritardando) with chords in bars, two note parts with a synth Claude shapes (plucks, leads, presets as data) and measures with its own ears, two mood macros (`darkness`, `instability`), a standing pace for transitions, delayed fades, smooth gestures with no clicks or jumps. a richer analog sound, arpeggios and ostinatos, more macros and the cinematic pulse are coming in Phase 2. Star or watch the repo to follow progress, or open an issue to help shape it.

---

## What it is

Moodscore is an open-source drone and ambient synthesizer you play by conversation.

You describe a mood ("colder", "something is getting closer", "let it breathe") and an AI agent connected over the [Model Context Protocol](https://modelcontextprotocol.io) turns it into musical gestures: a filter closing over forty seconds, a low voice fading in, the pitch starting to drift. The engine performs those gestures sample by sample, so the sound never stops to wait for the model.

It is built for:

- **Live mood sessions.** You talk, the agent conducts, the drone evolves in real time.
- **Scoring to picture.** Underscore a scene by describing what it should feel like, not which knob to turn.
- **Sound designers and filmmakers** who think in emotions first and parameters second.

## Why drones

An AI agent needs seconds to think. A melody can't wait that long; a drone can. Drone and ambient music live on slow, continuous change measured in tens of seconds, which is exactly the timescale an agent can direct live. Moodscore is designed around that fit.

## How it works

```
 you (voice or text)
        │
        ▼
 MCP client (Claude, or any MCP-capable agent)
        │  tool calls: morph, gesture, set_scene, listen…
        ▼
 ┌──────────────────── Moodscore ────────────────────┐
 │  MCP server                                        │
 │      │                                             │
 │  Mood macros ── tension, darkness, density,        │
 │      │          motion, distance, instability      │
 │      ▼                                             │
 │  Gesture director ── runs trajectories over time   │
 │      │                                             │
 │      ▼                                             │
 │  Sound engine (Faust DSP compiled to Rust)         │
 └──────────────────────┬─────────────────────────────┘
                        ▼
                    speakers
```

Three ideas carry the design:

1. **The agent conducts, the engine plays.** The agent never sends raw values in real time. It sends trajectories, such as *"take darkness to 0.8 over 30 seconds, ease in"*, and the engine performs them smoothly while the agent plans the next move.
2. **Mood, not knobs.** A handful of expressive macros each drive many synthesis parameters at once. "Darker" means something musical, not one filter.
3. **The agent can listen.** A `listen` tool reports what is actually coming out of the speakers (loudness, brightness, density, movement), so the agent can tell whether the sound matches what you asked for.

## A session, roughly

```text
You:    Start low and still. An empty hangar at night.
Agent:  set_scene("hangar")  →  two low drones, wide reverb, filter mostly closed

You:    Something is out there. Don't show it yet.
Agent:  morph(tension: 0.6, instability: 0.3, over: 45s, curve: ease_in)
        gesture(drone_4.level → 0.4, over: 60s)

You:    Now it's here.
Agent:  morph(tension: 0.9, darkness: 0.2, density: 0.8, over: 8s, curve: exponential)
```

*(Illustrative. See the draft API in [docs/MCP_TOOLS.md](docs/MCP_TOOLS.md).)*

## Planned sound engine

Inspired by the tradition of analog drone machines and early electronic instruments:

- **Six drone voices** with slow analog-style drift
- **Two melodic voices** for themes that rise out of the texture
- **Virtual-analog filters** with character, saturation, long delays and reverbs
- **Built-in imperfection**: detuning, wow, noise, all of it under macro control

## Stack (planned)

| Layer | Technology |
|---|---|
| DSP | [Faust](https://faust.grame.fr), compiled to Rust |
| Engine, director, server | Rust |
| Audio I/O | [cpal](https://github.com/RustAudio/cpal) (WASAPI/ASIO, CoreAudio, ALSA/JACK) |
| MCP | [rmcp](https://github.com/modelcontextprotocol/rust-sdk), the official Rust SDK |
| Mood mappings | JSON (`macros/default.json`), tunable without recompiling |

Moodscore will run as a standalone desktop app on Windows, macOS and Linux. Any MCP client can drive it; it is not tied to a single AI vendor.

## Quick start

You need [Rust](https://rustup.rs) 1.89 or newer. [Faust](https://github.com/grame-cncm/faust/releases) is optional: without it the build uses the pre-generated DSP code.

```sh
git clone https://github.com/JavierBaal/Moodscore.git
cd Moodscore
cargo build --release
```

Check that sound comes out of your speakers:

```sh
cargo run --release -- play
```

A forty-second demo: the drone slowly darkens, then opens up again. Turn your volume down first; it fades in, but drones carry a lot of low end.

Other commands: `render drone.wav 30` writes the same demo to a file; `params` lists the sound parameters.

## Play it with Claude

Moodscore is an MCP server. Your MCP client launches it, silent: nothing sounds until the agent calls `start`. From then on it plays through your default audio device until you ask the agent to stop, which fades it out with `release`, or until the client closes.

**Claude Code**

```sh
claude mcp add moodscore -- /path/to/Moodscore/target/release/moodscore serve
```

**Claude Desktop**: add this to `claude_desktop_config.json` (Settings → Developer → Edit Config), then restart Claude Desktop:

```json
{
  "mcpServers": {
    "moodscore": {
      "command": "/path/to/Moodscore/target/release/moodscore",
      "args": ["serve"]
    }
  }
}
```

On Windows the command is the full path to `moodscore.exe`, with doubled backslashes in JSON, e.g. `"H:\\Music\\Moodscore\\target\\release\\moodscore.exe"`.

Then talk to it: *"Start something low and calm. Now let it get darker, slowly, over the next minute."*

To retune the mood macros by ear, copy `macros/default.json`, edit it, and pass `--macros your-file.json` to `serve` or `play`.

## Documentation

- [Architecture](docs/ARCHITECTURE.md): components, threading and real-time rules
- [MCP tools (draft)](docs/MCP_TOOLS.md): the API an agent uses to play Moodscore
- [Roadmap](ROADMAP.md): what gets built, in what order
- [Contributing](CONTRIBUTING.md): how to help

## Contributing

The project is at the design stage, which is the best moment to influence it. DSP people, Rust audio developers, film composers and anyone who has opinions about how a mood should sound are all welcome. Start with [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
