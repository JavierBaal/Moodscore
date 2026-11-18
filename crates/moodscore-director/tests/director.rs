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
fn a_manual_gesture_takes_a_parameter_out_of_macro_control() {
    let mut d = director();
    d.gesture(0.0, "filter/cutoff", 2000.0, 4.0, Curve::Linear, 0.0)
        .unwrap();
    d.tick(4.0);
    assert!((value(&d, "filter/cutoff") - 2000.0).abs() < 0.5);
    // darkness keeps moving the other parameters, not the cutoff
    d.morph(4.0, &[("darkness", 0.9)], 0.0, Curve::Linear)
        .unwrap();
    d.tick(4.0 + 0.001);
    let state = d.state(4.001);
    let cutoff = state
        .params
        .iter()
        .find(|p| p.path == "filter/cutoff")
        .unwrap();
    assert_eq!(cutoff.owner, "macro:darkness");
}

#[test]
fn morph_reclaims_a_manual_parameter_without_a_jump() {
    let mut d = director();
    d.gesture(0.0, "filter/cutoff", 3000.0, 0.0, Curve::Linear, 0.0)
        .unwrap();
    d.tick(1.0);
    assert!((value(&d, "filter/cutoff") - 3000.0).abs() < 0.5);
    d.morph(1.0, &[("darkness", 1.0)], 10.0, Curve::EaseInOut)
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
