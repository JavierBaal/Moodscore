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

fn render_lr(part: &mut Part, events: &[(usize, Event)], frames: usize) -> (Vec<f32>, Vec<f32>) {
    let mut l = vec![0.0; frames];
    let mut r = vec![0.0; frames];
    part.render(events, &mut l, &mut r);
    (l, r)
}

fn energy(x: &[f32]) -> f32 {
    x.iter().map(|s| s * s).sum()
}

#[test]
fn echoes_follow_the_tempo() {
    // A short blip, echoed every quarter note at 120 BPM (0.5 s).
    let mut p = sine(|p| {
        p.set(P::AmpDecay, 0.02);
        p.set(P::AmpSustain, 0.0);
        p.set(P::DelayMix, 0.8);
        p.set(P::DelayDivision, 6.0);
        p.set(P::DelayWidth, 0.0);
        p.set(P::DelayFeedback, 0.5);
    });
    p.set_tempo(2.0);
    let (l, _) = render_lr(
        &mut p,
        &[(0, on(69)), (480, Event::NoteOff { note: 69 })],
        72_000,
    );
    let window = |t: f32| energy(&l[(t * 48_000.0) as usize..(t * 48_000.0) as usize + 2400]);
    assert!(window(0.5) > 100.0 * window(0.3), "first echo at 0.5 s");
    assert!(window(1.0) > 100.0 * window(0.8), "second echo at 1 s");
    assert!(window(1.0) < window(0.5), "each echo quieter");
}

#[test]
fn effects_keep_the_image_centred() {
    for (name, setup) in [
        (
            "chorus",
            (|p: &mut Patch| p.set(P::ChorusMix, 1.0)) as fn(&mut Patch),
        ),
        ("echo", |p: &mut Patch| {
            p.set(P::DelayMix, 0.7);
            p.set(P::DelayWidth, 1.0);
            p.set(P::DelayFeedback, 0.6);
        }),
        ("distortion", |p: &mut Patch| p.set(P::Distortion, 0.8)),
    ] {
        let mut p = sine(|p| {
            p.set(P::Osc1Shape, 2.0);
            setup(p);
        });
        let ev: Vec<_> = (0..8)
            .flat_map(|i| {
                [
                    (i * 12_000, on(57 + 2 * i as u8)),
                    (
                        i * 12_000 + 6000,
                        Event::NoteOff {
                            note: 57 + 2 * i as u8,
                        },
                    ),
                ]
            })
            .collect();
        let (l, r) = render_lr(&mut p, &ev, 144_000);
        let db = 10.0 * (energy(&l) / energy(&r)).log10();
        assert!(db.abs() < 1.0, "{name} leans {db:.2} dB");
    }
}

#[test]
fn the_send_follows_the_dry_sound() {
    let mut p = sine(|p| p.set(P::ReverbSend, 0.5));
    let n = 4800;
    let (mut dl, mut dr, mut sl, mut sr) = (vec![0.0; n], vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    p.render_bus(
        &[(0, on(69))],
        moodscore_synth::Bus {
            dry_l: &mut dl,
            dry_r: &mut dr,
            send_l: &mut sl,
            send_r: &mut sr,
        },
    );
    for i in 0..n {
        assert!((sl[i] - 0.5 * dl[i]).abs() < 1e-6 && (sr[i] - 0.5 * dr[i]).abs() < 1e-6);
    }
    assert!(energy(&dl) > 0.0);
}

#[test]
fn factory_presets_sound_at_a_sane_level() {
    for preset in moodscore_synth::Preset::factory() {
        let mut p = Part::new(SR, 9);
        p.set_patch(&preset.patch.0);
        p.set_tempo(2.0);
        // An A minor arpeggio in eighths, then a held note.
        let mut ev = Vec::new();
        for (i, note) in [57u8, 60, 64, 69, 64, 60, 57, 52].into_iter().enumerate() {
            ev.push((i * 12_000, on(note)));
            ev.push((i * 12_000 + 10_000, Event::NoteOff { note }));
        }
        ev.push((96_000, on(69)));
        ev.push((144_000, Event::NoteOff { note: 69 }));
        let (l, r) = render_lr(&mut p, &ev, 192_000);
        let peak = l.iter().chain(&r).fold(0.0f32, |m, s| m.max(s.abs()));
        let rms = (energy(&l[..96_000]) / 96_000.0).sqrt();
        println!("{}: peak {:.2}, rms {:.3}", preset.name, peak, rms);
        assert!(peak < 1.0 && peak > 0.1, "{} peak {peak}", preset.name);
        assert!(rms > 0.02, "{} too quiet", preset.name);
        assert!(
            l[190_000..].iter().all(|s| s.abs() < 0.05),
            "{} rings on",
            preset.name
        );
    }
}

// ---------------------------------------------------------------- analog, ladder

fn measure(
    part: &mut Part,
    events: &[(usize, Event)],
    frames: usize,
    spans: Vec<moodscore_ear::NoteSpan>,
) -> moodscore_ear::Report {
    let (l, r) = render_lr(part, events, frames);
    let audio = moodscore_ear::Audio {
        sample_rate: SR,
        left: l,
        right: r,
    };
    moodscore_ear::analyze(
        &audio,
        &moodscore_ear::Options {
            notes: spans,
            ..Default::default()
        },
    )
}

/// Eight separate A3s, one second each.
fn eight_notes(analog: f32) -> Vec<f32> {
    let mut p = sine(|p| {
        p.set(P::Analog, analog);
        p.set(P::PartVoices, 1.0);
    });
    let mut ev = Vec::new();
    let mut spans = Vec::new();
    for i in 0..8 {
        ev.push((i * 48_000 + 100, on(57)));
        ev.push((i * 48_000 + 40_000, Event::NoteOff { note: 57 }));
        spans.push(moodscore_ear::NoteSpan {
            start: (i * 48_000 + 100) as f64 / SR as f64,
            end: (i * 48_000 + 40_000) as f64 / SR as f64,
            notes: vec![57],
        });
    }
    let rep = measure(&mut p, &ev, 8 * 48_000, spans);
    rep.notes.iter().map(|n| n.tuning_cents.unwrap()).collect()
}

#[test]
fn analog_drift_makes_every_note_a_little_different() {
    let exact = eight_notes(0.0);
    assert!(
        exact.iter().all(|c| c.abs() < 0.5),
        "analog 0 is in tune: {exact:?}"
    );
    let alive = eight_notes(1.0);
    let mean = alive.iter().sum::<f32>() / alive.len() as f32;
    let spread =
        (alive.iter().map(|c| (c - mean).powi(2)).sum::<f32>() / alive.len() as f32).sqrt();
    assert!(spread > 1.5, "notes should differ: {alive:?}");
    assert!(
        alive.iter().all(|c| c.abs() < 12.0),
        "but stay near the note: {alive:?}"
    );
}

#[test]
fn analog_drift_wanders_while_a_note_holds() {
    let wobble = |analog: f32| {
        let mut p = sine(|p| p.set(P::Analog, analog));
        let spans = vec![moodscore_ear::NoteSpan {
            start: 0.0,
            end: 4.0,
            notes: vec![57],
        }];
        let rep = measure(&mut p, &[(0, on(57))], 4 * 48_000, spans);
        rep.notes[0].wobble_cents.unwrap()
    };
    assert!(wobble(0.0) < 0.3);
    let w = wobble(1.0);
    assert!(w > 0.4 && w < 6.0, "wander {w}");
}

#[test]
fn the_ladder_makes_a_deep_round_bass() {
    let harmonics = |filter: f32, slope: f32| {
        let mut p = sine(|p| {
            p.set(P::Osc1Shape, 2.0);
            p.set(P::FilterType, filter);
            p.set(P::FilterSlope, slope);
            p.set(P::Cutoff, 150.0); // about 2.5 x A#1
            p.set(P::Keytrack, 0.0);
            p.set(P::Resonance, 0.2);
        });
        let spans = vec![moodscore_ear::NoteSpan {
            start: 0.0,
            end: 2.0,
            notes: vec![34],
        }];
        let rep = measure(&mut p, &[(0, on(34))], 2 * 48_000, spans);
        rep.notes[0].harmonics_db.clone().unwrap()
    };
    let svf12 = harmonics(0.0, 12.0);
    let ladder = harmonics(3.0, 12.0);
    // H5 (index 3: harmonics 2..8) well below the gentle filter's.
    assert!(ladder[3] < -30.0, "ladder H5 {:?}", ladder);
    assert!(
        ladder[3] < svf12[3] - 10.0,
        "ladder {ladder:?} vs 12 dB {svf12:?}"
    );
}

#[test]
fn the_ensemble_widens_and_keeps_the_lows_mono() {
    let stereo = |ens: f32| {
        let mut p = sine(|p| {
            p.set(P::Osc1Shape, 2.0);
            p.set(P::Ensemble, ens);
        });
        let spans = vec![moodscore_ear::NoteSpan {
            start: 0.0,
            end: 4.0,
            notes: vec![45],
        }];
        let rep = measure(&mut p, &[(0, on(45))], 4 * 48_000, spans);
        (
            rep.metrics["mid_correlation"],
            rep.metrics["low_correlation"],
            rep.metrics["mid_balance_db"],
            rep.metrics["rms_dbfs"],
        )
    };
    let (dry_mid, _, _, dry_rms) = stereo(0.0);
    let (mid, low, bal, rms) = stereo(1.0);
    println!(
        "ensemble: mid {dry_mid} -> {mid}, low {low}, balance {bal}, level {dry_rms} -> {rms}"
    );
    assert!(dry_mid > 0.99);
    assert!(mid < 0.3, "ensemble width: mid correlation {mid}");
    assert!(low > 0.9, "lows stay mono: {low}");
    assert!(bal.abs() < 1.0, "centred: {bal}");
    assert!(
        (rms - dry_rms).abs() < 3.0,
        "about the same level: {dry_rms} -> {rms}"
    );
}
