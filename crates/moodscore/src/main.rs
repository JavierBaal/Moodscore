//! Moodscore command-line entry point.
//!
//! Phase 0 offers two commands to check the DSP pipeline end to end:
//!
//! ```text
//! moodscore params                     list the DSP parameters
//! moodscore render <out.wav> [secs]    render a demo drone to a WAV file
//! ```
//!
//! The demo slowly closes the filter: the first audible answer to
//! "get darker, slowly".

use std::env;
use std::process::ExitCode;

use moodscore_dsp::Dsp;

const SAMPLE_RATE: u32 = 48_000;
const BLOCK: usize = 256;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("params") => {
            print_params();
            ExitCode::SUCCESS
        }
        Some("render") => {
            let Some(path) = args.get(1) else {
                eprintln!("usage: moodscore render <out.wav> [seconds]");
                return ExitCode::FAILURE;
            };
            let seconds = match args.get(2).map(|s| s.parse::<f32>()) {
                None => 30.0,
                Some(Ok(s)) if s > 0.0 && s <= 600.0 => s,
                Some(_) => {
                    eprintln!("seconds must be a number between 0 and 600");
                    return ExitCode::FAILURE;
                }
            };
            match render_demo(path, seconds) {
                Ok(()) => {
                    println!("Rendered {seconds} s to {path}");
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("render failed: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!(
                "Moodscore {}\n\nusage:\n  moodscore params\n  moodscore render <out.wav> [seconds]",
                env!("CARGO_PKG_VERSION")
            );
            ExitCode::FAILURE
        }
    }
}

fn print_params() {
    let dsp = Dsp::new(SAMPLE_RATE);
    for p in dsp.params() {
        let unit = p.unit.as_deref().unwrap_or("");
        println!(
            "{:<20} {:>9.2} {:<6} range {} .. {}",
            p.path, p.init, unit, p.min, p.max
        );
    }
}

/// Render a drone whose filter closes from bright to dark over the whole duration.
///
/// The cutoff moves exponentially (equal steps in pitch, the way the ear
/// hears it), interpolated per block. Real gestures will come from the director.
fn render_demo(path: &str, seconds: f32) -> Result<(), hound::Error> {
    let mut dsp = Dsp::new(SAMPLE_RATE);
    let cutoff = dsp.param_id("filter/cutoff").expect("filter/cutoff");
    let (from, to) = (3_000.0f32, 180.0f32);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;

    let total = (seconds * SAMPLE_RATE as f32) as usize;
    let mut l = [0.0f32; BLOCK];
    let mut r = [0.0f32; BLOCK];
    let mut written = 0;
    while written < total {
        let t = written as f32 / total as f32;
        dsp.set(cutoff, from * (to / from).powf(t));
        dsp.compute(&mut l, &mut r);
        let n = BLOCK.min(total - written);
        for i in 0..n {
            writer.write_sample(l[i])?;
            writer.write_sample(r[i])?;
        }
        written += n;
    }
    writer.finalize()
}
