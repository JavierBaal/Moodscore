use moodscore_synth::params::{find, MOD_DESTS, MOD_SOURCES};
use moodscore_synth::{Event, Part, Patch, P};

const SR: u32 = 48_000;

fn on(note: u8) -> Event {
    Event::NoteOn {
        note,
        velocity: 1.0,
    }
}

fn sine(f: impl FnOnce(&mut Patch)) -> Part {
    let mut p = Patch::default();
    p.set(P::Osc1Shape, 0.0);
    p.set(P::Osc1Phase, 0.0);
    p.set(P::Cutoff, 20_000.0);
    p.set(P::Keytrack, 0.0);
    p.set(P::AmpAttack, 0.001);
    p.set(P::AmpSustain, 1.0);
    p.set(P::AmpRelease, 0.01);
    f(&mut p);
    let mut part = Part::new(SR, 1);
    part.set_patch(&p.0);
    part
}

fn route(p: &mut Patch, slot: usize, source: &str, dest: &str, amount: f32) {
    let base = P::Mod1Source as usize + 3 * (slot - 1);
    p.0[base] = MOD_SOURCES.iter().position(|s| *s == source).unwrap() as f32;
    p.0[base + 1] = MOD_DESTS.iter().position(|s| *s == dest).unwrap() as f32;
    p.0[base + 2] = amount;
}

fn render(part: &mut Part, events: &[(usize, Event)], frames: usize) -> Vec<f32> {
    let mut l = vec![0.0; frames];
    let mut r = vec![0.0; frames];
    part.render(events, &mut l, &mut r);
    l
}

/// Average frequency in a stretch, from positive-going zero crossings.
fn freq(x: &[f32]) -> f32 {
    let mut first = None;
    let mut last = 0.0;
    let mut n = 0;
    for i in 1..x.len() {
        if x[i - 1] < 0.0 && x[i] >= 0.0 {
            let t = (i - 1) as f32 + x[i - 1] / (x[i - 1] - x[i]);
            if first.is_none() {
                first = Some(t);
            } else {
                n += 1;
            }
            last = t;
        }
    }
    n as f32 * SR as f32 / (last - first.unwrap())
}

fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt()
}

#[test]
fn notes_land_on_their_exact_sample() {
    let mut a = sine(|_| {});
    let mut b = sine(|_| {});
    let x = render(&mut a, &[(100, on(60))], 4096);
    let y = render(&mut b, &[(357, on(60))], 4096);
    assert!(x[..100].iter().all(|&s| s == 0.0));
    for k in 0..3000 {
        assert_eq!(x[100 + k], y[357 + k], "sample {k} after the note");
    }
}

#[test]
fn polyphony_and_stealing() {
    let mut p = sine(|_| {});
    let ev: Vec<_> = (0..4).map(|i| (i * 100, on(60 + i as u8))).collect();
    render(&mut p, &ev, 1000);
    assert_eq!(p.active_voices(), 4);
    // A fifth note steals the oldest, and nothing clicks.
    let out = render(&mut p, &[(0, on(72))], 4800);
    assert_eq!(p.active_voices(), 4);
    let step = out
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0, f32::max);
    assert!(step < 0.2, "steal jumped {step}");
    // Two-voice polyphony.
    let mut p = sine(|p| p.set(P::PartVoices, 2.0));
    render(&mut p, &[(0, on(60)), (10, on(64)), (20, on(67))], 1000);
    assert_eq!(p.active_voices(), 2);
}

#[test]
fn legato_slides_without_restarting() {
    let make = |legato: f32| {
        sine(|p| {
            p.set(P::PartVoices, 1.0);
            p.set(P::PartLegato, legato);
            p.set(P::AmpDecay, 0.1);
            p.set(P::AmpSustain, 0.2);
        })
    };
    let level_after_second_note = |legato: f32| {
        let mut p = make(legato);
        let out = render(&mut p, &[(0, on(60)), (24_000, on(64))], 26_400);
        rms(&out[24_100..24_600])
    };
    let leg = level_after_second_note(1.0);
    let retrig = level_after_second_note(0.0);
    assert!(retrig > 2.0 * leg, "retrigger {retrig} vs legato {leg}");

    // Letting go of the top key goes back to the one still held.
    let mut p = make(1.0);
    render(&mut p, &[(0, on(57)), (1000, on(69))], 9600);
    let out = render(&mut p, &[(0, Event::NoteOff { note: 69 })], 9600);
    assert_eq!(p.active_voices(), 1);
    assert!((freq(&out[2400..]) - 220.0).abs() < 1.0);
    let out = render(&mut p, &[(0, Event::NoteOff { note: 57 })], 9600);
    assert!(out[4800..].iter().all(|&s| s == 0.0), "released");
}

#[test]
fn glide_slides_in_its_time() {
    let mut p = sine(|p| {
        p.set(P::PartVoices, 1.0);
        p.set(P::PartLegato, 1.0);
        p.set(P::PartGlide, 0.3);
    });
    render(&mut p, &[(0, on(57))], 9600);
    let out = render(&mut p, &[(0, on(69))], 48_000);
    let early = freq(&out[480..2400]);
    assert!(early > 230.0 && early < 400.0, "sliding: {early}");
    assert!((freq(&out[24_000..]) - 440.0).abs() < 1.0);
}

#[test]
fn delayed_vibrato() {
    let mut p = sine(|p| {
        p.set(P::Lfo1Rate, 6.0);
        p.set(P::Lfo1Delay, 0.4);
        p.set(P::Lfo1Fade, 0.1);
        // 0.025 of the bend's 24 semitones: ±0.6 semitones.
        route(p, 1, "lfo1", "pitch/bend", 0.025);
    });
    let out = render(&mut p, &[(0, on(69))], 48_000);
    let spread = |a: usize, b: usize| {
        let fs: Vec<f32> = (a..b)
            .step_by(1200)
            .map(|i| freq(&out[i..i + 1200]))
            .collect();
        fs.iter().cloned().fold(0.0, f32::max) - fs.iter().cloned().fold(1e9, f32::min)
    };
    assert!(spread(1200, 18_000) < 0.5, "steady before the delay");
    let s = spread(26_400, 46_800);
    assert!(s > 20.0 && s < 40.0, "vibrato depth {s} Hz");
}

#[test]
fn synced_lfo_follows_the_tempo() {
    // Tremolo on a quarter note at 120 BPM: two dips a second.
    let mut p = sine(|p| {
        p.set(P::Lfo1Sync, 1.0);
        p.set(P::Lfo1Division, 6.0); // 1/4
        p.set(P::Lfo1Shape, 0.0);
        route(p, 1, "lfo1", "voice/gain", -0.5);
    });
    p.set_tempo(2.0);
    let out = render(&mut p, &[(0, on(69))], 96_000);
    let env: Vec<f32> = out.chunks(480).map(rms).collect(); // 10 ms
    let mut dips = 0;
    for w in env.windows(3) {
        if w[1] < w[0] && w[1] <= w[2] && w[1] < 0.5 * env.iter().cloned().fold(0.0, f32::max) {
            dips += 1;
        }
    }
    assert_eq!(dips, 4, "dips in two seconds");
}

#[test]
fn matrix_routes_expression_and_velocity() {
    let brightness = |expr: f32, vel: f32| {
        let mut p = sine(|p| {
            p.set(P::Osc1Shape, 2.0);
            p.set(P::Cutoff, 300.0);
            p.set(P::Expression, expr);
            route(p, 1, "expression", "filter/cutoff", 0.4);
            route(p, 2, "velocity", "filter/cutoff", 0.3);
        });
        let out = render(
            &mut p,
            &[(
                0,
                Event::NoteOn {
                    note: 45,
                    velocity: vel,
                },
            )],
            9600,
        );
        let x = &out[4800..];
        x.windows(2).map(|w| (w[1] - w[0]).powi(2)).sum::<f32>()
            / x.iter().map(|s| s * s).sum::<f32>()
    };
    assert!(brightness(1.0, 0.0) > 3.0 * brightness(0.0, 0.0));
    assert!(brightness(0.0, 1.0) > 3.0 * brightness(0.0, 0.0));
    assert!(find("pitch/bend").is_some());
}
