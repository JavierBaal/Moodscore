mod common;

use common::*;
use moodscore_synth::osc::{Osc, Shape};

const SR: f32 = 48_000.0;
const LIMIT: f32 = 10_000.0;

fn naive(shape: &Shape, f0: f32, n: usize) -> Vec<f32> {
    let mut p = 0.0f32;
    (0..n)
        .map(|_| {
            p = (p + f0 / SR).fract();
            shape.value(p)
        })
        .collect()
}

fn blep(shape: &Shape, f0: f32, sync: f32, n: usize) -> Vec<f32> {
    let mut o = Osc::default();
    (0..n).map(|_| o.next(shape, f0 / SR, sync)).collect()
}

fn naive_sync(shape: &Shape, f0: f32, sync: f32, n: usize) -> Vec<f32> {
    let (mut p, mut m) = (0.0f32, 0.0f32);
    (0..n)
        .map(|_| {
            m += f0 / SR;
            p += f0 * sync / SR;
            if m >= 1.0 {
                m -= 1.0;
                p = m * sync;
            }
            p = p.fract();
            shape.value(p)
        })
        .collect()
}

fn alias(x: &[f32], f0: f32) -> f32 {
    worst_alias_db(&spectrum_db(x), f0, SR, 4, LIMIT)
}

/// Aliasing that folds back below 10 kHz (where it is audible as
/// inharmonic tones), against a naive oscillator.
#[test]
fn oscillators_are_band_limited() {
    let n = 8192;
    // (morph, f0, ceiling in dB)
    for (morph, f0, ceiling) in [
        (1.0, 440.0, -90.0),
        (1.0, 3520.0, -62.0),
        (2.0, 440.0, -60.0),
        (2.0, 3520.0, -44.0),
        (3.0, 440.0, -60.0),
        (3.0, 3520.0, -44.0),
        (2.5, 1760.0, -50.0),
    ] {
        let s = Shape::new(morph, 0.5);
        let a = alias(&naive(&s, f0, n), f0);
        let b = alias(&blep(&s, f0, 1.0, n), f0);
        assert!(b < ceiling, "morph {morph} at {f0} Hz: {b:.1} dB");
        assert!(
            b < a - 20.0,
            "morph {morph} at {f0} Hz: {b:.1} vs naive {a:.1}"
        );
    }
    for sync in [1.7f32, 3.3] {
        let s = Shape::new(2.0, 0.5);
        let a = alias(&naive_sync(&s, 440.0, sync, n), 440.0);
        let b = alias(&blep(&s, 440.0 * sync, sync, n), 440.0);
        assert!(
            b < -50.0 && b < a - 20.0,
            "sync {sync}: {b:.1} vs naive {a:.1}"
        );
    }
}

// ---------------------------------------------------------------- voice

use moodscore_synth::{Patch, Voice, P};

fn patch(f: impl FnOnce(&mut Patch)) -> Patch {
    let mut p = Patch::default();
    f(&mut p);
    p
}

fn sine_patch() -> Patch {
    patch(|p| {
        p.set(P::Osc1Shape, 0.0);
        p.set(P::Osc1Phase, 0.0);
        p.set(P::Cutoff, 20_000.0);
        p.set(P::Keytrack, 0.0);
        p.set(P::AmpAttack, 0.001);
        p.set(P::AmpRelease, 0.005);
        p.set(P::AmpSustain, 1.0);
    })
}

/// Render a voice for `frames`, in chunks of `chunk`.
fn run(v: &mut Voice, p: &Patch, frames: usize) -> (Vec<f32>, Vec<f32>) {
    let mut l = vec![0.0; frames];
    let mut r = vec![0.0; frames];
    for c in (0..frames).step_by(64) {
        let e = (c + 64).min(frames);
        v.render(&p.0, 1.0, &mut l[c..e], &mut r[c..e]);
    }
    (l, r)
}

fn max_step(x: &[f32]) -> f32 {
    x.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0, f32::max)
}

#[test]
fn notes_start_and_stop_without_clicks() {
    let p = sine_patch();
    let mut v = Voice::new(48_000, 1);
    v.start(&p.0, 57, 1.0, None); // A3, 220 Hz
    let (on, _) = run(&mut v, &p, 4800);
    // A 220 Hz sine at full scale moves at most 2π·220/48000 per sample.
    let sine_step = std::f32::consts::TAU * 220.0 / 48_000.0;
    assert!(max_step(&on) < sine_step * 1.2, "attack {}", max_step(&on));
    v.release();
    let (off, _) = run(&mut v, &p, 4800);
    assert!(
        max_step(&off) < sine_step * 1.2,
        "release {}",
        max_step(&off)
    );
    assert!(!v.is_active(), "voice ends after its release");
    assert!(off[4000..].iter().all(|&s| s == 0.0));
    // A retrigger during the release rises from where it is.
    let mut v = Voice::new(48_000, 1);
    v.start(&p.0, 57, 1.0, None);
    let (mut a, _) = run(&mut v, &p, 2400);
    v.release();
    let (b, _) = run(&mut v, &p, 100);
    v.start(&p.0, 57, 1.0, None);
    let (c, _) = run(&mut v, &p, 2400);
    a.extend(b);
    a.extend(c);
    assert!(max_step(&a) < sine_step * 1.2, "retrigger {}", max_step(&a));
}

#[test]
fn latency_is_fixed_and_short() {
    let p = sine_patch();
    let mut v = Voice::new(48_000, 1);
    v.start(&p.0, 69, 1.0, None);
    let (l, _) = run(&mut v, &p, 256);
    let first = l.iter().position(|s| s.abs() > 1e-6).unwrap();
    assert!(first <= 16, "first sound after {first} samples");
}

#[test]
fn unison_is_balanced_and_spread() {
    let p = patch(|p| {
        p.set(P::Osc1Unison, 5.0);
        p.set(P::Osc1Spread, 1.0);
        p.set(P::Osc1Detune, 30.0);
    });
    let mut v = Voice::new(48_000, 7);
    v.start(&p.0, 45, 1.0, None);
    let (l, r) = run(&mut v, &p, 96_000);
    let e = |x: &[f32]| x[4800..].iter().map(|s| s * s).sum::<f32>();
    let db = 10.0 * (e(&l) / e(&r)).log10();
    assert!(db.abs() < 1.0, "unison leans {db:.2} dB");
    let corr = l[4800..]
        .iter()
        .zip(&r[4800..])
        .map(|(a, b)| a * b)
        .sum::<f32>()
        / (e(&l) * e(&r)).sqrt();
    assert!(
        corr < 0.95,
        "spread copies should decorrelate L and R ({corr:.3})"
    );
    // Without spread, the sound is mono.
    let p = patch(|p| {
        p.set(P::Osc1Unison, 5.0);
        p.set(P::Osc1Spread, 0.0);
    });
    let mut v = Voice::new(48_000, 7);
    v.start(&p.0, 45, 1.0, None);
    let (l, r) = run(&mut v, &p, 4800);
    assert!(l.iter().zip(&r).all(|(a, b)| (a - b).abs() < 1e-5));
}

#[test]
fn oversampling_tames_the_shapers() {
    use moodscore_synth::dsp::{drive, fold, Drive, Oversampler};
    let n = 8192;
    let f0 = 1760.0;
    let x: Vec<f32> = (0..n)
        .map(|i| (std::f32::consts::TAU * f0 * i as f32 / SR).sin() * 0.9)
        .collect();
    // Hard saturation of a high note: the worst case for aliasing.
    let shaper = |v: f32| drive(fold(v, 0.7), 0.8);
    let plain: Vec<f32> = x.iter().map(|&s| shaper(s)).collect();
    let mut o = Oversampler::default();
    let mut d = Drive::default();
    let over: Vec<f32> = x
        .iter()
        .map(|&s| o.process(s, |v| d.process(fold(v, 0.7), 0.8)))
        .collect();
    let a = alias(&plain, f0);
    let b = alias(&over[64..], f0);
    println!("shaper aliasing: plain {a:.1} dB, 2x {b:.1} dB");
    assert!(b < -45.0 && b < a - 25.0, "2x: {b:.1} dB vs {a:.1} dB");
}

#[test]
fn the_filter_envelope_opens_the_sound() {
    let bright = |amount: f32| {
        let p = patch(|p| {
            p.set(P::Cutoff, 300.0);
            p.set(P::FilterEnv, amount);
            p.set(P::FenvDecay, 2.0);
            p.set(P::FenvSustain, 1.0);
        });
        let mut v = Voice::new(48_000, 3);
        v.start(&p.0, 45, 1.0, None);
        let (l, _) = run(&mut v, &p, 9600);
        // Energy of the first difference: a rough brightness.
        l[4800..]
            .windows(2)
            .map(|w| (w[1] - w[0]).powi(2))
            .sum::<f32>()
            / l[4800..].iter().map(|s| s * s).sum::<f32>()
    };
    assert!(bright(4.0) > 4.0 * bright(0.0));
}

/// How long a fully loaded voice takes per sample (run with --ignored --nocapture).
#[test]
#[ignore]
fn cost_of_a_heavy_voice() {
    let p = patch(|p| {
        p.set(P::Osc1Unison, 7.0);
        p.set(P::Osc2Unison, 7.0);
        p.set(P::Osc2Level, 0.7);
        p.set(P::Osc1Sync, 2.3);
        p.set(P::SubLevel, 0.5);
        p.set(P::NoiseLevel, 0.1);
        p.set(P::Drive, 0.5);
        p.set(P::Fold, 0.3);
        p.set(P::FilterSlope, 24.0);
    });
    let mut v = Voice::new(48_000, 1);
    v.start(&p.0, 45, 1.0, None);
    let n = 480_000;
    let t = std::time::Instant::now();
    let (l, _) = run(&mut v, &p, n);
    let ns = t.elapsed().as_nanos() as f64 / n as f64;
    println!(
        "heavy voice: {ns:.0} ns/sample ({:.1}% of one core at 48 kHz) {}",
        ns * 48_000.0 / 1e7,
        l[n / 2]
    );
}
