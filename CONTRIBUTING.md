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

Will be documented once Phase 0 lands. Expect:

- Rust (stable, latest)
- The [Faust compiler](https://github.com/grame-cncm/faust/releases) on your `PATH`

## License

By contributing, you agree that your contributions are dual-licensed under MIT and Apache-2.0, as described in the [README](README.md#license).

## Conduct

Be kind, be specific, critique the work and not the person.
