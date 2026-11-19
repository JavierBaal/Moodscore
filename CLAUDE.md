# Moodscore: instructions for coding agents

Moodscore is a drone engine for live, mood-driven film scoring over MCP. An AI agent conducts (sends long trajectories); the engine plays (performs them sample-accurately).

Read before working: `README.md`, `docs/ARCHITECTURE.md`, `docs/MCP_TOOLS.md`, `ROADMAP.md`.

## Language

All code, comments, docs, commit messages and issues are in English.

## Stack

- DSP in Faust (`dsp/*.dsp`), compiled to Rust at build time
- Rust workspace under `crates/`: `moodscore-dsp` (Faust: drones, room), `moodscore-synth` (note voices, Rust), `moodscore-engine`, `moodscore-director`, `moodscore-mcp`, `moodscore` (binary)
- Audio I/O: cpal. MCP: rmcp (official Rust SDK).

## Hard rules

1. **Audio thread is sacred.** No allocation, locks, I/O, logging or waiting inside the audio callback. Communicate only through preallocated lock-free queues.
2. **No jumps.** Every parameter change goes through smoothing. A new gesture starts from the parameter's current value.
3. **Macro mappings are data**, not code.
4. **The DSP and synth crates know nothing about moods or MCP.** Keep layers separate: dsp, synth ← engine ← director ← mcp.
5. Keep the MCP API in `docs/MCP_TOOLS.md` in sync with the implementation.

## Commands

- `cargo test --all`
- `cargo run --release -p moodscore -- params`
- `cargo run --release -- render out.wav 30` (offline, same director/engine path as live)
- `cargo run --release -- play` (live demo through the speakers)
- `python3 scripts/mcp_smoke_test.py target/release/moodscore` (MCP end to end, headless)
- After editing `dsp/`: rebuild with `MOODSCORE_UPDATE_GENERATED=1` and commit `crates/moodscore-dsp/generated/moodscore_dsp.rs` (needs Faust ≥ 2.88; see CONTRIBUTING.md).

## Conventions

- `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` must pass.
- Prefer small, focused changes. Update `ROADMAP.md` when a phase item is done.
