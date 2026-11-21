use moodscore_director::{mapping, Curve, Director, DirectorError, DEFAULT_MAPPINGS};
use moodscore_engine::ParamInfo;

fn params() -> Vec<ParamInfo> {
    moodscore_dsp::Dsp::new(48_000).params().to_vec()
}

fn director() -> Director {
    Director::with_default_mappings(params()).expect("default mappings are valid")
}

fn value(d: &Director, path: &str) -> f32 {
    let id = d.params().iter().position(|p| p.path == path).unwrap();
    d.values().nth(id).unwrap()
}

/// Sample a parameter while running the director forward.
fn trace(d: &mut Director, path: &str, from: f64, to: f64, step: f64) -> Vec<f32> {
    let mut out = Vec::new();
    let mut t = from;
    while t <= to + 1e-9 {
        d.tick(t);
        out.push(value(d, path));
        t += step;
    }
    out
}

#[test]
fn default_mappings_are_valid() {
    let macros = mapping::load(DEFAULT_MAPPINGS, &params()).unwrap();
    assert!(macros.iter().any(|m| m.name == "darkness"));
}

#[test]
fn starts_at_the_macro_defaults() {
    let d = director();
    let cutoff = value(&d, "filter/cutoff");
    // darkness default 0.4: 4200 * (140/4200)^0.4
    let expected = 4200.0 * (140.0f32 / 4200.0).powf(0.4);
    assert!((cutoff - expected).abs() < 1.0, "{cutoff} vs {expected}");
}

#[test]
fn darkness_closes_the_filter_smoothly_and_arrives() {
    let mut d = director();
    d.morph(0.0, &[("darkness", 1.0)], 10.0, Curve::EaseInOut)
        .unwrap();
    let tr = trace(&mut d, "filter/cutoff", 0.0, 12.0, 0.01);
    assert!(tr.windows(2).all(|w| w[1] <= w[0] + 1e-3), "not monotonic");
    assert!((tr.last().unwrap() - 140.0).abs() < 0.5);
    // ease_in_out: half way in time is half way in ratio
    let mid = tr[500];
    let expected = 4200.0 * (140.0f32 / 4200.0).powf(0.4 + 0.6 * 0.5);
    assert!(
        (mid - expected).abs() / expected < 0.02,
        "{mid} vs {expected}"
    );
}

#[test]
fn changing_your_mind_mid_gesture_never_jumps() {
    let mut d = director();
    d.morph(0.0, &[("darkness", 1.0)], 10.0, Curve::Linear)
        .unwrap();
    d.tick(5.0);
    let before = value(&d, "filter/cutoff");
    d.morph(5.0, &[("darkness", 0.0)], 10.0, Curve::Linear)
        .unwrap();
    d.tick(5.0);
    let after = value(&d, "filter/cutoff");
    assert!((before - after).abs() < 1e-3, "{before} -> {after}");
}

#[test]
fn a_hand_set_parameter_stays_put_when_its_macro_moves() {
    let mut d = director();
    d.gesture(0.0, "space/reverb_mix", 0.55, 4.0, Curve::Linear, 0.0)
        .unwrap();
    d.tick(4.0);
    let reso_before = value(&d, "filter/resonance");
    // darkness moves everything it drives except the hand-set reverb
    d.morph(4.0, &[("darkness", 0.9)], 2.0, Curve::Linear)
        .unwrap();
    d.tick(6.0);
    assert!((value(&d, "space/reverb_mix") - 0.55).abs() < 1e-5);
    assert!(value(&d, "filter/resonance") > reso_before);
    let state = d.state(6.0);
    let owner = |path: &str| {
        state
            .params
            .iter()
            .find(|p| p.path == path)
            .unwrap()
            .owner
            .clone()
    };
    assert_eq!(owner("space/reverb_mix"), "manual");
    assert_eq!(owner("filter/resonance"), "macro:darkness");
}

#[test]
fn morph_with_reclaim_takes_a_hand_set_parameter_back_without_a_jump() {
    let mut d = director();
    d.gesture(0.0, "filter/cutoff", 3000.0, 0.0, Curve::Linear, 0.0)
        .unwrap();
    d.tick(1.0);
    assert!((value(&d, "filter/cutoff") - 3000.0).abs() < 0.5);
    d.morph_with(1.0, &[("darkness", 1.0)], 10.0, Curve::EaseInOut, true)
        .unwrap();
    let tr = trace(&mut d, "filter/cutoff", 1.0, 12.0, 0.01);
    assert!((tr[0] - 3000.0).abs() < 1.0, "jumped to {}", tr[0]);
    let max_step = tr
        .windows(2)
        .map(|w| (w[1] / w[0]).ln().abs())
        .fold(0.0, f32::max);
    assert!(max_step < 0.02, "step ratio {max_step}");
    assert!((tr.last().unwrap() - 140.0).abs() < 0.5);
}

#[test]
fn start_in_delays_a_gesture() {
    let mut d = director();
    let start = value(&d, "drone_1/pitch");
    d.gesture(0.0, "drone_1/pitch", 110.0, 2.0, Curve::Linear, 3.0)
        .unwrap();
    d.tick(2.9);
    assert_eq!(value(&d, "drone_1/pitch"), start);
    d.tick(5.0);
    assert!((value(&d, "drone_1/pitch") - 110.0).abs() < 0.01);
}

#[test]
fn instant_means_a_short_glide() {
    let mut d = director();
    d.gesture(0.0, "master/gain", 0.0, 0.0, Curve::Linear, 0.0)
        .unwrap();
    d.tick(0.01);
    assert!(value(&d, "master/gain") > 0.0, "jumped");
}

#[test]
fn bad_requests_are_rejected_clearly() {
    let mut d = director();
    assert_eq!(
        d.morph(0.0, &[("joy", 0.5)], 1.0, Curve::Linear),
        Err(DirectorError::UnknownMacro("joy".into()))
    );
    assert!(matches!(
        d.morph(0.0, &[("darkness", 1.5)], 1.0, Curve::Linear),
        Err(DirectorError::OutOfRange { .. })
    ));
    assert!(matches!(
        d.gesture(0.0, "filter/cutoff", 99_999.0, 1.0, Curve::Linear, 0.0),
        Err(DirectorError::OutOfRange { .. })
    ));
    assert!(matches!(
        d.gesture(0.0, "nope", 1.0, 1.0, Curve::Linear, 0.0),
        Err(DirectorError::UnknownParam(_))
    ));
    assert!(d
        .morph(0.0, &[("darkness", 0.5)], -1.0, Curve::Linear)
        .is_err());
}

#[test]
fn state_reports_motion() {
    let mut d = director();
    d.morph(0.0, &[("darkness", 0.9)], 20.0, Curve::Linear)
        .unwrap();
    d.tick(5.0);
    let s = d.state(5.0);
    let dark = &s.macros[0];
    assert_eq!(dark.name, "darkness");
    assert_eq!(dark.target, 0.9);
    assert!((dark.seconds_remaining - 15.0).abs() < 0.01);
    let json = serde_json::to_string(&s).unwrap();
    assert!(json.contains("\"owner\":\"macro:darkness\""));
}

#[test]
fn invalid_mappings_are_rejected() {
    let p = params();
    let bad_param =
        r#"{"macros":{"x":{"default":0.5,"targets":[{"param":"nope","from":0,"to":1}]}}}"#;
    assert!(mapping::load(bad_param, &p).is_err());
    let out_of_range =
        r#"{"macros":{"x":{"default":0.5,"targets":[{"param":"master/gain","from":0,"to":7}]}}}"#;
    assert!(mapping::load(out_of_range, &p).is_err());
    let twice = r#"{"macros":{
        "a":{"default":0.5,"targets":[{"param":"master/gain","from":0,"to":1}]},
        "b":{"default":0.5,"targets":[{"param":"master/gain","from":0,"to":1}]}}}"#;
    assert!(mapping::load(twice, &p).is_err());
}

#[test]
fn instability_moves_drift_and_detune_but_not_darkness() {
    let mut d = director();
    let cutoff_before = value(&d, "filter/cutoff");
    d.morph(0.0, &[("instability", 1.0)], 5.0, Curve::Linear)
        .unwrap();
    d.tick(5.0);
    assert!((value(&d, "drone_1/drift") - 1.0).abs() < 1e-4);
    assert!((value(&d, "drone_1/detune") - 22.0).abs() < 1e-3);
    assert_eq!(value(&d, "filter/cutoff"), cutoff_before);

    let state = d.state(5.0);
    let detune = state
        .params
        .iter()
        .find(|p| p.path == "drone_1/detune")
        .unwrap();
    assert_eq!(detune.owner, "macro:instability");
}

#[test]
fn pace_defaults_and_partial_updates() {
    let mut d = director();
    assert_eq!(d.pace().over, 10.0);
    assert_eq!(d.pace().curve, Curve::EaseInOut);

    let p = d.set_pace(Some(45.0), None).unwrap();
    assert_eq!((p.over, p.curve), (45.0, Curve::EaseInOut));
    let p = d.set_pace(None, Some(Curve::Linear)).unwrap();
    assert_eq!((p.over, p.curve), (45.0, Curve::Linear));
    // "short and direct": zero is allowed and means the shortest safe glide
    assert_eq!(d.set_pace(Some(0.0), None).unwrap().over, 0.0);
}

#[test]
fn bad_pace_is_rejected_and_changes_nothing() {
    let mut d = director();
    assert!(matches!(
        d.set_pace(None, None),
        Err(DirectorError::Invalid(_))
    ));
    assert!(d.set_pace(Some(-1.0), Some(Curve::Linear)).is_err());
    assert!(d.set_pace(Some(f32::NAN), None).is_err());
    assert!(d.set_pace(Some(10_000.0), None).is_err());
    assert_eq!(d.pace().over, 10.0);
    assert_eq!(d.pace().curve, Curve::EaseInOut);
}

#[test]
fn state_reports_pace() {
    let mut d = director();
    d.set_pace(Some(30.0), Some(Curve::EaseIn)).unwrap();
    let json = serde_json::to_string(&d.state(0.0)).unwrap();
    assert!(json.contains("\"pace\":{\"over\":30.0,\"curve\":\"ease_in\"}"));
}

/// A director with a custom mapping, for testing mapping features.
fn director_with(json: &str) -> Director {
    let p = params();
    let macros = mapping::load(json, &p).expect("valid test mapping");
    Director::new(p, macros)
}

#[test]
fn wildcards_drive_every_matching_parameter() {
    let d = director();
    let state = d.state(0.0);
    let drifts: Vec<_> = state
        .params
        .iter()
        .filter(|p| p.path.starts_with("drone_") && p.path.ends_with("/drift"))
        .collect();
    assert!(drifts.len() >= 2, "expected several drone voices");
    for p in drifts {
        assert_eq!(p.owner, "macro:instability", "{}", p.path);
    }
}

#[test]
fn add_macros_stack_on_top_of_the_setter() {
    let mut d = director_with(
        r#"{"macros":{
            "dark": {"default":0.0,"targets":[{"param":"filter/resonance","from":0.2,"to":0.4}]},
            "tense":{"default":0.0,"targets":[{"param":"filter/resonance","from":0.0,"to":0.3,"mode":"add"}]}
        }}"#,
    );
    assert!((value(&d, "filter/resonance") - 0.2).abs() < 1e-5);

    d.morph(0.0, &[("tense", 1.0)], 1.0, Curve::Linear).unwrap();
    d.tick(1.0);
    assert!((value(&d, "filter/resonance") - 0.5).abs() < 1e-5);

    d.morph(1.0, &[("dark", 1.0)], 1.0, Curve::Linear).unwrap();
    d.tick(2.0);
    assert!((value(&d, "filter/resonance") - 0.7).abs() < 1e-5);

    let state = d.state(2.0);
    let reso = state
        .params
        .iter()
        .find(|p| p.path == "filter/resonance")
        .unwrap();
    assert_eq!(reso.owner, "macro:dark+tense");
}

#[test]
fn stacked_values_are_clamped_and_adds_start_from_the_default() {
    // nothing sets the resonance: the add starts from its default (0.3)
    let mut d = director_with(
        r#"{"macros":{"tense":{"default":0.0,"targets":[
            {"param":"filter/resonance","from":0.0,"to":0.9,"mode":"add"}]}}}"#,
    );
    assert!((value(&d, "filter/resonance") - 0.3).abs() < 1e-5);
    d.morph(0.0, &[("tense", 1.0)], 0.0, Curve::Linear).unwrap();
    d.tick(1.0);
    assert!((value(&d, "filter/resonance") - 0.95).abs() < 1e-5); // range max
}

#[test]
fn a_shared_parameter_moves_without_jumps_when_another_macro_joins() {
    let mut d = director_with(
        r#"{"macros":{
            "dark": {"default":0.5,"targets":[{"param":"filter/resonance","from":0.2,"to":0.4}]},
            "tense":{"default":0.0,"targets":[{"param":"filter/resonance","from":0.0,"to":0.3,"mode":"add"}]}
        }}"#,
    );
    d.morph(0.0, &[("dark", 1.0)], 10.0, Curve::Linear).unwrap();
    d.morph(5.0, &[("tense", 1.0)], 10.0, Curve::Linear)
        .unwrap();
    let t = trace(&mut d, "filter/resonance", 0.0, 20.0, 0.01);
    let biggest_step = t
        .windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .fold(0.0, f32::max);
    assert!(biggest_step < 0.001, "jump of {biggest_step}");
}

#[test]
fn mapping_rules_for_add_and_wildcards() {
    let p = params();
    let two_setters = r#"{"macros":{
        "a":{"default":0.5,"targets":[{"param":"filter/resonance","from":0,"to":0.5}]},
        "b":{"default":0.5,"targets":[{"param":"filter/resonance","from":0,"to":0.5}]}}}"#;
    assert!(mapping::load(two_setters, &p).is_err());
    let exp_add = r#"{"macros":{"a":{"default":0.5,"targets":[
        {"param":"filter/cutoff","from":1,"to":100,"curve":"exp","mode":"add"}]}}}"#;
    assert!(mapping::load(exp_add, &p).is_err());
    let no_match = r#"{"macros":{"a":{"default":0.5,"targets":[
        {"param":"voice_*/drift","from":0,"to":1}]}}}"#;
    assert!(mapping::load(no_match, &p).is_err());
    let typo = r#"{"macros":{"a":{"default":0.5,"targets":[
        {"param":"filter/resonance","from":0,"to":0.5,"mdoe":"add"}]}}}"#;
    assert!(mapping::load(typo, &p).is_err());
}

// ---------------------------------------------------------------- harmony

use moodscore_director::harmony::{hz_to_midi, parse_note, Chord, Key};
use moodscore_director::{HarmonyPlan, Length, StartOn};

fn plan(key: &str, mode: &str, chords: &[(&str, f32)], looping: bool) -> HarmonyPlan {
    let lengths: Vec<(&str, Length)> = chords
        .iter()
        .map(|&(c, s)| (c, Length::Seconds(s)))
        .collect();
    plan_in(key, mode, &lengths, looping)
}

fn plan_in(key: &str, mode: &str, chords: &[(&str, Length)], looping: bool) -> HarmonyPlan {
    let k = Key::new(key, mode).unwrap();
    HarmonyPlan {
        progression: chords
            .iter()
            .map(|(c, l)| (Chord::parse(c, &k).unwrap(), *l))
            .collect(),
        key: k,
        looping,
        pedal: None,
        glide: 2.0,
        curve: Curve::EaseInOut,
        start_on: StartOn::Now,
    }
}

/// Pitch classes sounding on the voices (MIDI rounded), bass first.
fn pitch_classes(d: &Director) -> Vec<u8> {
    (1..=6)
        .map(|v| (hz_to_midi(value(d, &format!("drone_{v}/pitch"))).round() as i32 % 12) as u8)
        .collect()
}

#[test]
fn harmony_walks_the_progression_and_loops() {
    let mut d = director();
    d.harmony(
        0.0,
        plan("A", "aeolian", &[("i", 10.0), ("VI", 10.0)], true),
        false,
    )
    .unwrap();
    d.tick(5.0);
    let s = d.state(5.0);
    let h = s.harmony.as_ref().unwrap();
    assert_eq!(h.chord.name, "Am");
    assert_eq!(h.next.as_ref().unwrap().name, "F");
    assert!((h.seconds_to_next.unwrap() - 5.0).abs() < 1e-3);

    d.tick(13.0); // VI started at 10 s, 2 s glide: arrived
    let mut pcs = pitch_classes(&d);
    assert_eq!(pcs[0], 5, "bass on F");
    pcs.sort();
    pcs.dedup();
    assert_eq!(pcs, vec![0, 5, 9], "F A C");

    d.tick(23.0); // looped back to i
    let mut pcs = pitch_classes(&d);
    assert_eq!(pcs[0], 9, "bass back on A");
    pcs.sort();
    pcs.dedup();
    assert_eq!(pcs, vec![0, 4, 9], "A C E");
    assert_eq!(d.state(23.0).harmony.unwrap().position, "1/2");
}

#[test]
fn chord_changes_glide_without_jumps() {
    let mut d = director();
    d.harmony(
        0.0,
        plan("A", "aeolian", &[("i", 4.0), ("VII", 4.0)], true),
        false,
    )
    .unwrap();
    for v in 1..=6 {
        let path = format!("drone_{v}/pitch");
        let t = trace(&mut d.clone(), &path, 0.0, 12.0, 0.005);
        let worst = t
            .windows(2)
            .map(|w| (w[1] / w[0]).ln().abs())
            .fold(0.0, f32::max);
        assert!(worst < 0.01, "{path} jumped (step ratio {worst})");
    }
}

#[test]
fn a_plan_that_does_not_loop_holds_its_last_chord() {
    let mut d = director();
    d.harmony(
        0.0,
        plan("C", "major", &[("I", 3.0), ("IV", 3.0)], false),
        false,
    )
    .unwrap();
    d.tick(60.0);
    let s = d.state(60.0);
    let h = s.harmony.unwrap();
    assert_eq!(h.chord.name, "F");
    assert!(h.next.is_none() && h.seconds_to_next.is_none());
    assert_eq!(pitch_classes(&d)[0], 5);
}

#[test]
fn harmony_respects_a_hand_tuned_voice_until_reclaimed() {
    let mut d = director();
    d.gesture(0.0, "drone_6/pitch", 300.0, 0.0, Curve::Linear, 0.0)
        .unwrap();
    d.harmony(1.0, plan("A", "aeolian", &[("VI", 30.0)], true), false)
        .unwrap();
    d.tick(5.0);
    assert!((value(&d, "drone_6/pitch") - 300.0).abs() < 1e-3);
    let s = d.state(5.0);
    let p6 = s.params.iter().find(|p| p.path == "drone_6/pitch").unwrap();
    assert_eq!(p6.owner, "manual");
    assert_eq!(p6.held_from, vec!["harmony".to_string()]);
    let p5 = s.params.iter().find(|p| p.path == "drone_5/pitch").unwrap();
    assert_eq!(p5.owner, "harmony");

    d.harmony(5.0, plan("A", "aeolian", &[("VI", 30.0)], true), true)
        .unwrap();
    d.tick(8.0);
    let pc6 = (hz_to_midi(value(&d, "drone_6/pitch")).round() as i32 % 12) as u8;
    assert!([0, 5, 9].contains(&pc6), "reclaimed voice joins the chord");
}

#[test]
fn a_pedal_holds_the_bass_and_stop_frees_the_voices() {
    let mut d = director();
    let mut p = plan(
        "A",
        "aeolian",
        &[("i", 5.0), ("VI", 5.0), ("VII", 5.0)],
        true,
    );
    p.pedal = Some(parse_note("A").unwrap());
    d.harmony(0.0, p, false).unwrap();
    for t in [3.0, 8.0, 13.0] {
        d.tick(t);
        assert!(
            (value(&d, "drone_1/pitch") - 55.0).abs() < 0.01,
            "bass at {t}s"
        );
    }
    d.stop_harmony();
    let before = value(&d, "drone_3/pitch");
    d.tick(40.0);
    assert!(
        (value(&d, "drone_3/pitch") - before).abs() < 1e-3,
        "voices stay put"
    );
    let s = d.state(40.0);
    assert!(s.harmony.is_none());
    assert!(s
        .params
        .iter()
        .filter(|p| p.path.ends_with("/pitch"))
        .all(|p| p.owner == "manual"));
    // and a gesture works on them as usual
    d.gesture(40.0, "drone_3/pitch", 100.0, 0.0, Curve::Linear, 0.0)
        .unwrap();
}

// ---------------------------------------------------------------- tempo

/// The chord sounding at `t` and the seconds to the next one.
fn chord_at(d: &mut Director, t: f64) -> (String, Option<f32>) {
    d.tick(t);
    let h = d.state(t).harmony.unwrap();
    (h.chord.name, h.seconds_to_next)
}

#[test]
fn chords_in_bars_follow_the_tempo() {
    let mut d = director();
    d.set_tempo(0.0, Some(138.0), None, 0.0, Curve::Linear, true)
        .unwrap();
    let bar = 4.0 * 60.0 / 138.0; // 1.739 s
    d.harmony(
        0.0,
        plan_in(
            "B",
            "aeolian",
            &[("i", Length::Bars(2.0)), ("VI", Length::Beats(2.0))],
            true,
        ),
        false,
    )
    .unwrap();
    let (c, left) = chord_at(&mut d, 0.5);
    assert_eq!(c, "Bm");
    assert!((left.unwrap() as f64 - (2.0 * bar - 0.5)).abs() < 1e-3);
    assert_eq!(chord_at(&mut d, 2.0 * bar + 0.01).0, "G");
    // VI lasts two beats, half a bar; then the loop comes back to i.
    assert_eq!(chord_at(&mut d, 2.5 * bar + 0.01).0, "Bm");
    // After 10 laps the change still lands on the beat: no drift.
    let lap = 2.5 * bar;
    assert_eq!(chord_at(&mut d, 10.0 * lap - 0.001).0, "G");
    assert_eq!(chord_at(&mut d, 10.0 * lap + 0.001).0, "Bm");
}

#[test]
fn a_ritardando_stretches_the_chords() {
    let mut d = director();
    d.harmony(
        0.0,
        plan_in(
            "A",
            "aeolian",
            &[("i", Length::Bars(1.0)), ("VI", Length::Bars(1.0))],
            true,
        ),
        false,
    )
    .unwrap();
    // 120 BPM: a bar every 2 s. Halve the tempo over 4 bars from 2 s on.
    let secs = d
        .tempo()
        .ramp_seconds_for(2.0, 60.0, None, 16.0, Curve::Linear);
    d.set_tempo(2.0, Some(60.0), None, secs, Curve::Linear, false)
        .unwrap();
    let end = d.tempo().time_at(4.0 + 16.0);
    assert!((end - (2.0 + secs)).abs() < 1e-9);
    // F, Am, F, Am fill the four bars of the ramp, so F comes back right
    // as it ends: later than four bars at the old tempo would have taken.
    assert!(end > 2.0 + 8.0);
    assert_eq!(chord_at(&mut d, end - 0.01).0, "Am");
    assert_eq!(chord_at(&mut d, end + 0.01).0, "F");
    let s = d.state(end + 0.01);
    assert_eq!(s.tempo.bpm, 60.0);
    assert!(s.tempo.ramping_to.is_none());
    assert_eq!(s.tempo.bar, 6);
    // Halfway through, the ramp is reported.
    let s = d.state(2.0 + secs / 2.0);
    assert_eq!(s.tempo.ramping_to, Some(60.0));
    assert!(s.tempo.bpm < 120.0 && s.tempo.bpm > 60.0);
}

#[test]
fn a_tempo_change_mid_chord_keeps_what_was_played() {
    let mut d = director();
    d.harmony(
        0.0,
        plan_in(
            "A",
            "aeolian",
            &[("i", Length::Bars(2.0)), ("VI", Length::Bars(2.0))],
            true,
        ),
        false,
    )
    .unwrap();
    // One bar (2 s) in, double the tempo: the second bar of i takes 1 s.
    d.set_tempo(2.0, Some(240.0), None, 0.0, Curve::Linear, false)
        .unwrap();
    assert_eq!(chord_at(&mut d, 2.99).0, "Am");
    assert_eq!(chord_at(&mut d, 3.01).0, "F");
}

#[test]
fn plans_can_wait_for_the_downbeat() {
    let mut d = director();
    let mut p = plan_in("C", "major", &[("IV", Length::Bars(1.0))], true);
    p.start_on = StartOn::Bar;
    // 120 BPM 4/4 since 0: next downbeat at 4 s.
    d.harmony(2.7, p, false).unwrap();
    d.tick(3.0);
    let s = d.state(3.0);
    let h = s.harmony.unwrap();
    assert!((h.starts_in.unwrap() - 1.0).abs() < 1e-3);
    assert!(h.next.is_none());
    // Nothing has moved yet.
    let rest = value(&d, "drone_1/pitch");
    assert!((rest - 55.0).abs() < 0.01);
    d.tick(4.5);
    assert!(value(&d, "drone_1/pitch") < rest, "bass gliding to F");
    assert!(d.state(4.5).harmony.unwrap().starts_in.is_none());
}

#[test]
fn tempo_is_validated_and_reported() {
    let mut d = director();
    assert!(d
        .set_tempo(0.0, Some(5.0), None, 0.0, Curve::Linear, false)
        .is_err());
    d.set_tempo(
        1.0,
        Some(90.0),
        Some("3/4".parse().unwrap()),
        0.0,
        Curve::Linear,
        false,
    )
    .unwrap();
    let s = d.state(1.0 + 2.0); // 3 beats at 90 BPM = one 3/4 bar
    assert_eq!(s.tempo.meter, "3/4");
    assert_eq!(s.tempo.bar, 2);
    assert!((s.tempo.beat - 1.0).abs() < 0.011);
    assert!((s.tempo.seconds_per_bar - 2.0).abs() < 1e-3);
}

#[test]
fn plans_can_start_on_a_given_bar() {
    let mut d = director();
    // 120 BPM 4/4 from 0: bar 3 starts at 4 s.
    let mut p = plan_in("C", "major", &[("IV", Length::Bars(1.0))], true);
    p.start_on = StartOn::AtBar(3);
    d.harmony(0.5, p.clone(), false).unwrap();
    let h = d.state(1.0).harmony.unwrap();
    assert!((h.starts_in.unwrap() - 3.0).abs() < 1e-3);
    d.tick(3.9);
    assert!(
        (value(&d, "drone_1/pitch") - 55.0).abs() < 0.01,
        "still waiting"
    );
    d.tick(4.5);
    assert!(value(&d, "drone_1/pitch") < 55.0, "gliding to F from bar 3");
    // A bar that has passed is refused, and says where we are.
    p.start_on = StartOn::AtBar(2);
    let e = d.harmony(5.0, p, false).unwrap_err().to_string();
    assert!(e.contains("bar 3 now"), "{e}");
}

#[test]
fn slash_chords_move_the_bass() {
    let mut d = director();
    let mut p = plan_in(
        "D",
        "ionian",
        &[("I", Length::Seconds(2.0)), ("I/C#", Length::Seconds(2.0))],
        false,
    );
    p.glide = 0.2;
    d.harmony(0.0, p, false).unwrap();
    d.tick(1.0);
    let before = value(&d, "drone_1/pitch");
    d.tick(5.0);
    let after = value(&d, "drone_1/pitch");
    // Pitches are in Hz: back to pitch classes.
    let pc = |hz: f32| (69.0 + 12.0 * (hz / 440.0).log2()).round() as i32 % 12;
    assert_eq!(pc(before), 2, "D in the bass ({before} Hz)");
    assert_eq!(pc(after), 1, "C# in the bass ({after} Hz)");
    assert_eq!(d.state(5.0).harmony.unwrap().chord.name, "D/C#");
}

// ---------------------------------------------------------------- sequences

#[test]
fn the_sequencer_sees_the_chords_ahead() {
    let mut d = director();
    d.set_tempo(0.0, Some(120.0), None, 0.0, Curve::Linear, true)
        .unwrap();
    assert!(d.chord_at(0.0).is_none());
    let chords = [
        ("i", Length::Bars(1.0)),
        ("VI", Length::Bars(1.0)),
        ("iv", Length::Bars(1.0)),
        ("V", Length::Bars(1.0)),
    ];
    d.harmony(0.0, plan_in("Bb", "aeolian", &chords, true), false)
        .unwrap();
    d.tick(0.1);
    let name = |t: f64| d.chord_at(t).unwrap().chord.name;
    // A bar is 2 s: the sequencer can ask about any time ahead.
    assert_eq!(
        [name(0.5), name(2.5), name(4.5), name(6.5), name(8.5)],
        ["Bbm", "Gb", "Ebm", "F", "Bbm"]
    );
    // The same chord on the next lap is a new chord.
    assert_ne!(d.chord_at(0.5).unwrap().id, d.chord_at(8.5).unwrap().id);
    // A new plan waiting for bar 3: until then, the current one walks on.
    let mut p = plan_in("A", "dorian", &[("i", Length::Bars(1.0))], true);
    p.start_on = StartOn::AtBar(3);
    d.tick(0.5);
    d.harmony(0.5, p, false).unwrap();
    assert_eq!(d.chord_at(3.0).unwrap().chord.name, "Gb");
    assert_eq!(d.chord_at(4.1).unwrap().chord.name, "Am");
}

#[test]
fn a_sequence_plays_through_the_conductor() {
    use moodscore_director::sequence::{parse_rhythm, Order, Pattern, Source};
    use moodscore_director::Conductor;
    use std::time::{Duration, Instant};

    let (engine, mut renderer) = moodscore_engine::new(48_000);
    let mut d = Director::with_default_mappings(engine.params().to_vec()).unwrap();
    d.set_tempo(0.0, Some(240.0), None, 0.0, Curve::Linear, true)
        .unwrap();
    let c = Conductor::start(d, engine.clone());
    // Play the audio in real time, as a sound card would.
    let audio = std::thread::spawn(move || {
        let (mut l, mut r) = (vec![0.0f32; 480], vec![0.0f32; 480]);
        let t0 = Instant::now();
        let mut done = 0u64;
        while t0.elapsed() < Duration::from_millis(1500) {
            let due = (t0.elapsed().as_secs_f64() * 48_000.0) as u64;
            while done + 480 <= due {
                renderer.render(&mut l, &mut r);
                done += 480;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    });
    let pattern = Pattern {
        source: Source::Arp {
            order: Order::Up,
            low: 57,
            high: 69,
            notes: Some(vec![57, 60, 64]),
        },
        steps: parse_rhythm("x").unwrap(),
        rate: 0.25,
        gate: 0.5,
        velocity: 0.8,
        accent: 0.15,
        swing: 0.0,
        legato: false,
    };
    // Without harmony, an arpeggio over the chord sounding is refused.
    let mut over_chord = pattern.clone();
    over_chord.source = Source::Arp {
        order: Order::Up,
        low: 57,
        high: 69,
        notes: None,
    };
    assert!(c
        .sequence(0, over_chord, "arp".into(), StartOn::Now, None)
        .is_err());
    let state = c
        .sequence(0, pattern, "arp".into(), StartOn::Beat, Some(1.0))
        .unwrap();
    assert!(state.ends_in.is_some());
    assert_eq!(c.sequences_of(0).len(), 1);
    // A sixteenth every 62.5 ms: notes sound within the bar (1 s).
    let t0 = Instant::now();
    let mut heard = false;
    while t0.elapsed() < Duration::from_millis(900) {
        heard |= engine.active_notes(0) > 0;
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(heard, "the sequence never sounded");
    // One bar, then it is done and gone.
    std::thread::sleep(Duration::from_millis(500));
    assert!(c.sequences_of(0).is_empty());
    c.stop();
    audio.join().unwrap();
}

// ------------------------------------------------- plans and tempo that wait

#[test]
fn a_plan_waiting_for_its_bar_lets_the_current_one_walk_on() {
    let mut d = director();
    d.set_tempo(0.0, Some(120.0), None, 0.0, Curve::Linear, true)
        .unwrap();
    let four = [
        ("i", Length::Bars(1.0)),
        ("VI", Length::Bars(1.0)),
        ("iv", Length::Bars(1.0)),
        ("V", Length::Bars(1.0)),
    ];
    d.harmony(0.0, plan_in("A", "aeolian", &four, true), false)
        .unwrap();
    d.tick(0.5);
    // From bar 4 (6 s): D dorian.
    let mut p = plan_in("D", "dorian", &[("i", Length::Bars(2.0))], true);
    p.start_on = StartOn::AtBar(4);
    d.harmony(0.5, p, false).unwrap();
    // The A minor plan still walks: bar 2 is VI, bar 3 iv.
    d.tick(2.5);
    assert_eq!(d.state(2.5).harmony.unwrap().chord.name, "F");
    let st = d.state(4.5).harmony.unwrap();
    assert_eq!(st.chord.name, "Dm");
    assert!(st
        .then
        .as_ref()
        .is_some_and(|n| n.chord.name == "Dm" && n.starts_in > 1.4));
    // The sequencer sees the change coming.
    assert_eq!(d.chord_at(5.9).unwrap().chord.name, "Dm");
    assert_eq!(d.chord_at(6.1).unwrap().key.mode.to_string(), "dorian");
    let before = d.chord_at(5.0).unwrap().id;
    d.tick(6.2);
    let st = d.state(6.2).harmony.unwrap();
    assert!(st.key.contains("dorian") && st.then.is_none());
    assert_ne!(d.chord_at(6.2).unwrap().id, before);
}

#[test]
fn tempo_can_wait_for_a_bar() {
    let mut d = director();
    d.set_tempo(0.0, Some(120.0), None, 0.0, Curve::Linear, true)
        .unwrap();
    // At bar 3 (4 s), 60 BPM: from then a bar lasts 4 s.
    let at = d
        .tempo_at(
            0.1,
            StartOn::AtBar(3),
            Some(60.0),
            None,
            moodscore_director::tempo::Ramp::Seconds(0.0),
            Curve::Linear,
            false,
        )
        .unwrap();
    assert!((at - 4.0).abs() < 1e-9);
    let st = d.state(0.2).tempo;
    assert_eq!(st.bpm, 120.0);
    assert_eq!(st.changes_ahead.len(), 1);
    assert_eq!(st.changes_ahead[0].bar, 3);
    assert!((d.tempo().time_of_bar(4) - 8.0).abs() < 1e-9);
    // A plan in bars, set now, already knows: bar 3's chord lasts 4 s.
    let two = [("i", Length::Bars(1.0)), ("iv", Length::Bars(1.0))];
    d.harmony(0.2, plan_in("A", "aeolian", &two, true), false)
        .unwrap();
    // i 0.2–2.2, iv 2.2–4.4 (its last fifth of a bar at the new tempo),
    // then i over a whole slow bar, 4.4–8.4.
    assert_eq!(d.chord_at(4.3).unwrap().chord.name, "Dm");
    assert_eq!(d.chord_at(8.3).unwrap().chord.name, "Am");
    assert_eq!(d.chord_at(8.5).unwrap().chord.name, "Dm");
    d.tick(5.0);
    let st = d.state(5.0).tempo;
    assert_eq!(st.bpm, 60.0);
    assert!(st.changes_ahead.is_empty());
    // Bad values are refused, waiting or not.
    assert!(d
        .tempo_at(
            5.0,
            StartOn::AtBar(5),
            Some(900.0),
            None,
            moodscore_director::tempo::Ramp::Seconds(0.0),
            Curve::Linear,
            false
        )
        .is_err());
}
