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
