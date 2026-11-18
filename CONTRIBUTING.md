# Contributing to Moodscore

Thanks for your interest. Moodscore is at the design stage, so the most valuable contributions right now are ideas, critique and sound.

## Ways to help

- **Design feedback.** Read [ARCHITECTURE.md](docs/ARCHITECTURE.md) and [MCP_TOOLS.md](docs/MCP_TOOLS.md) and open an issue with what's wrong or missing.
- **DSP.** Faust developers: drone voices, filters with character, reverbs that can hold a minute-long tail.
- **Rust audio.** Real-time-safe engine code, cross-platform audio I/O, build tooling.
- **Musical judgment.** What should "tension" actually do to a drone? Composers and sound designers, this one is for you.

## Before you write code

Open an issue first for anything larger than a small fix, so we agree on the approach before you spend time on it.

## Ground rules for code

- **Real-time safety.** Nothing on the audio thread may allocate, lock, do I/O or wait. See [ARCHITECTURE.md](docs/ARCHITECTURE.md#real-time-rules). PRs that break this will not be merged, however good they sound.
- **No hard jumps.** Every parameter change reaching the DSP goes through smoothing.
- **Mappings are data.** Macro → parameter curves live in data files, not hard-coded, so they can be tuned by ear.
- Format with `cargo fmt` and keep `cargo clippy` clean.
- Small, focused PRs with a clear description of what you changed and how it sounds (a short audio clip is welcome).

## Development setup

- **Rust** (stable, latest) via [rustup](https://rustup.rs).
- **Faust** ([releases](https://github.com/grame-cncm/faust/releases), 2.88 or newer) on your `PATH`, only if you change the DSP.

```sh
cargo test --all
cargo run --release -p moodscore -- render drone.wav 30
```

### Working on the DSP

The sound lives in `dsp/moodscore.dsp`. On build, `crates/moodscore-dsp/build.rs` compiles it with Faust if it can find it (on `PATH`, or via the `FAUST` environment variable). Without Faust it falls back to the pre-generated code in `crates/moodscore-dsp/generated/`, so everyone else can build without installing Faust.

When you change the DSP, regenerate that file and commit it with your change:

```sh
# macOS / Linux
MOODSCORE_UPDATE_GENERATED=1 cargo build -p moodscore-dsp
```

```powershell
# Windows PowerShell
$env:MOODSCORE_UPDATE_GENERATED=1; cargo build -p moodscore-dsp; Remove-Item Env:MOODSCORE_UPDATE_GENERATED
```

The DSP crate only exposes raw synthesis parameters. Moods, gestures and smoothing over time belong to the director and engine.

## License

By contributing, you agree that your contributions are dual-licensed under MIT and Apache-2.0, as described in the [README](README.md#license).

## Conduct

Be kind, be specific, critique the work and not the person.
