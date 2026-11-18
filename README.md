# Moodscore

**A drone engine for live, mood-driven film scoring over MCP.**

Tell it how the scene should feel. It moves the sound there.

> **Status: Phase 0.** The DSP pipeline works: a single drone voice, written in Faust, compiled to Rust, rendered to a WAV file. The live engine and the MCP server are next. Star or watch the repo to follow progress, or open an issue to help shape it.

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

Moodscore will run as a standalone desktop app on Windows, macOS and Linux. Any MCP client can drive it; it is not tied to a single AI vendor.

## Quick start

You need [Rust](https://rustup.rs) (stable). [Faust](https://github.com/grame-cncm/faust/releases) is optional: without it the build uses the pre-generated DSP code.

```sh
git clone https://github.com/JavierBaal/Moodscore.git
cd Moodscore
cargo run --release -p moodscore -- params              # list the DSP parameters
cargo run --release -p moodscore -- render drone.wav 30  # 30 s drone that slowly darkens
```

Open `drone.wav` and listen: a low, slightly unstable drone whose filter closes over thirty seconds. It's the first, hard-coded answer to *"get darker, slowly"*.

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
