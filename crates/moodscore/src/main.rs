//! Moodscore command-line entry point.
//!
//! ```text
//! moodscore serve [--macros FILE] [--no-audio]    MCP server over stdio, playing live
//! moodscore play [SECONDS] [--macros FILE]        live demo without MCP (check your audio)
//! moodscore render OUT.wav [SECONDS] [--macros FILE]  offline demo to a WAV file
//! moodscore params                                list the DSP parameters
//! ```
//!
//! In `serve` mode stdout carries the MCP protocol: all messages go to stderr.

use std::process::ExitCode;
use std::thread;
use std::time::Duration;

use moodscore_director::{mapping, Conductor, Curve, Director, MacroDef, DEFAULT_MAPPINGS};
use moodscore_engine::ParamInfo;

const USAGE: &str = "\
usage:
  moodscore serve [--macros FILE] [--no-audio]        MCP server over stdio, playing live
  moodscore play [SECONDS] [--macros FILE]            live demo without MCP (default 40 s)
  moodscore render OUT.wav [SECONDS] [--macros FILE]  offline demo to a WAV file (default 30 s)
  moodscore params                                    list the DSP parameters";

const OFFLINE_SAMPLE_RATE: u32 = 48_000;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let macros_file = match take_option(&mut args, "--macros") {
        Ok(v) => v,
        Err(e) => return fail(e),
    };
    let no_audio = take_flag(&mut args, "--no-audio");

    let result = match args.first().map(String::as_str) {
        Some("serve") => serve(macros_file.as_deref(), no_audio),
        Some("play") => {
            seconds_arg(args.get(1), 40.0).and_then(|s| play(s, macros_file.as_deref()))
        }
        Some("render") => match args.get(1) {
            Some(path) => {
                seconds_arg(args.get(2), 30.0).and_then(|s| render(path, s, macros_file.as_deref()))
            }
            None => Err("render needs an output file, e.g. moodscore render drone.wav".into()),
        },
        Some("params") => {
            print_params();
            Ok(())
        }
        _ => {
            eprintln!("Moodscore {}\n\n{USAGE}", env!("CARGO_PKG_VERSION"));
            return ExitCode::FAILURE;
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => fail(e),
    }
}

fn fail(e: impl std::fmt::Display) -> ExitCode {
    eprintln!("moodscore: {e}");
    ExitCode::FAILURE
}

fn take_option(args: &mut Vec<String>, name: &str) -> Result<Option<String>> {
    match args.iter().position(|a| a == name) {
        None => Ok(None),
        Some(i) if i + 1 < args.len() => {
            let v = args.remove(i + 1);
            args.remove(i);
            Ok(Some(v))
        }
        Some(_) => Err(format!("{name} needs a value").into()),
    }
}

fn take_flag(args: &mut Vec<String>, name: &str) -> bool {
    match args.iter().position(|a| a == name) {
        Some(i) => {
            args.remove(i);
            true
        }
        None => false,
    }
}

fn seconds_arg(arg: Option<&String>, default: f32) -> Result<f32> {
    match arg.map(|s| s.parse::<f32>()) {
        None => Ok(default),
        Some(Ok(s)) if s > 0.0 && s <= 3600.0 => Ok(s),
        Some(_) => Err("seconds must be a number between 0 and 3600".into()),
    }
}

fn load_macros(file: Option<&str>, params: &[ParamInfo]) -> Result<Vec<MacroDef>> {
    let json = match file {
        Some(path) => std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read macro mappings {path}: {e}"))?,
        None => DEFAULT_MAPPINGS.to_string(),
    };
    Ok(mapping::load(&json, params)?)
}

/// Open the audio device and start the director against it.
///
/// With `silent`, the engine starts at zero output gain and waits for `start`.
fn start_live(
    macros_file: Option<&str>,
    silent: bool,
) -> Result<(Conductor, moodscore_engine::audio::AudioOutput)> {
    // Validate the mappings before any sound starts.
    let params = moodscore_dsp::Dsp::new(OFFLINE_SAMPLE_RATE)
        .params()
        .to_vec();
    let macros = load_macros(macros_file, &params)?;

    let mut conductor = None;
    let (_engine, output) = moodscore_engine::audio::start_default_output(|engine| {
        if silent {
            engine.fade_to(0.0, 0.0);
        }
        let director = Director::new(engine.params().to_vec(), macros);
        conductor = Some(Conductor::start(director, engine.clone()));
    })?;
    let conductor = conductor.expect("prepare callback ran");
    eprintln!(
        "moodscore: playing on {} ({} Hz, {} ch)",
        output.device, output.sample_rate, output.channels
    );
    Ok((conductor, output))
}

/// Run the engine in real time without an audio device (testing, CI).
fn start_headless(macros_file: Option<&str>) -> Result<Conductor> {
    let (engine, mut renderer) = moodscore_engine::new(OFFLINE_SAMPLE_RATE);
    engine.fade_to(0.0, 0.0); // silent until `start`, like live `serve`
    let macros = load_macros(macros_file, engine.params())?;
    let conductor = Conductor::start(Director::new(engine.params().to_vec(), macros), engine);
    thread::Builder::new()
        .name("moodscore-headless-audio".into())
        .spawn(move || {
            let block = (OFFLINE_SAMPLE_RATE / 100) as usize; // 10 ms
            let (mut l, mut r) = (vec![0.0f32; block], vec![0.0f32; block]);
            let start = std::time::Instant::now();
            let mut rendered = 0u64;
            loop {
                renderer.render(&mut l, &mut r);
                rendered += block as u64;
                let due = Duration::from_secs_f64(rendered as f64 / OFFLINE_SAMPLE_RATE as f64);
                if let Some(wait) = due.checked_sub(start.elapsed()) {
                    thread::sleep(wait);
                }
            }
        })?;
    eprintln!("moodscore: running without audio output");
    Ok(conductor)
}

fn serve(macros_file: Option<&str>, no_audio: bool) -> Result<()> {
    let (conductor, _output) = if no_audio {
        (start_headless(macros_file)?, None)
    } else {
        let (c, o) = start_live(macros_file, true)?;
        (c, Some(o))
    };
    eprintln!("moodscore: MCP server ready on stdio (silent until `start`)");
    let runtime = tokio::runtime::Runtime::new()?;
    let result = runtime.block_on(moodscore_mcp::serve_stdio(conductor.clone()));
    conductor.stop();
    eprintln!("moodscore: client disconnected, stopping");
    result
}

/// A scripted performance: darken slowly, then open up again.
const SCRIPT: [(f32, f32, f32, Curve); 3] = [
    // (start at fraction of total, darkness, over fraction of total, curve)
    (0.05, 0.95, 0.55, Curve::EaseInOut),
    (0.65, 0.15, 0.3, Curve::EaseOut),
    (0.97, 0.4, 0.03, Curve::Linear),
];

fn play(seconds: f32, macros_file: Option<&str>) -> Result<()> {
    let (conductor, _output) = start_live(macros_file, false)?;
    eprintln!(
        "moodscore: demo for {seconds:.0} s: darker, slowly, then opening up. Ctrl+C to stop."
    );
    let mut next = 0;
    let mut last_report = -10.0;
    while conductor.now() < seconds as f64 {
        let t = conductor.now() as f32 / seconds;
        if next < SCRIPT.len() && t >= SCRIPT[next].0 {
            let (_, darkness, over, curve) = SCRIPT[next];
            conductor.morph(&[("darkness", darkness)], over * seconds, curve, false)?;
            eprintln!(
                "moodscore: darkness -> {darkness} over {:.0} s",
                over * seconds
            );
            next += 1;
        }
        if conductor.now() - last_report >= 5.0 {
            last_report = conductor.now();
            let s = conductor.state();
            let dark = s
                .macros
                .iter()
                .find(|m| m.name == "darkness")
                .map_or(0.0, |m| m.value);
            let cutoff = s
                .params
                .iter()
                .find(|p| p.path == "filter/cutoff")
                .map_or(0.0, |p| p.value);
            eprintln!(
                "  {:>5.1} s  darkness {dark:.2}  cutoff {cutoff:>6.0} Hz  level {:>6.1} dBFS",
                conductor.now(),
                conductor.engine().levels().rms_dbfs()
            );
        }
        thread::sleep(Duration::from_millis(20));
    }
    conductor.stop();
    Ok(())
}

/// Render the same scripted performance offline, through the real director
/// and engine, in simulated time.
fn render(path: &str, seconds: f32, macros_file: Option<&str>) -> Result<()> {
    let sr = OFFLINE_SAMPLE_RATE;
    let (engine, mut renderer) = moodscore_engine::new(sr);
    let macros = load_macros(macros_file, engine.params())?;
    let mut director = Director::new(engine.params().to_vec(), macros);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: sr,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;

    let control_frames = (sr / moodscore_director::CONTROL_RATE_HZ) as usize;
    let total = (seconds * sr as f32) as usize;
    let mut l = vec![0.0f32; control_frames];
    let mut r = vec![0.0f32; control_frames];
    let mut written = 0;
    let mut next = 0;
    while written < total {
        let now = written as f64 / sr as f64;
        let t = now as f32 / seconds;
        if next < SCRIPT.len() && t >= SCRIPT[next].0 {
            let (_, darkness, over, curve) = SCRIPT[next];
            director.morph(now, &[("darkness", darkness)], over * seconds, curve)?;
            next += 1;
        }
        director.tick(now);
        for (id, v) in director.values().enumerate() {
            engine.set_target(id, v);
        }
        let n = control_frames.min(total - written);
        renderer.render(&mut l[..n], &mut r[..n]);
        for i in 0..n {
            writer.write_sample(l[i])?;
            writer.write_sample(r[i])?;
        }
        written += n;
    }
    writer.finalize()?;
    println!("Rendered {seconds} s to {path}");
    Ok(())
}

fn print_params() {
    let dsp = moodscore_dsp::Dsp::new(OFFLINE_SAMPLE_RATE);
    for p in dsp.params() {
        let unit = p.unit.as_deref().unwrap_or("");
        println!(
            "{:<20} {:>9.2} {:<6} range {} .. {}",
            p.path, p.init, unit, p.min, p.max
        );
    }
}
