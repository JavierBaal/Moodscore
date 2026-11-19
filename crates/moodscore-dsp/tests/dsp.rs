use moodscore_dsp::{Dsp, Scale, NUM_OUTPUTS};

const SR: u32 = 48_000;
const BLOCK: usize = 256;

fn render(dsp: &mut Dsp, seconds: f32) -> (Vec<f32>, Vec<f32>) {
    let frames = (seconds * SR as f32) as usize;
    let mut l_all = Vec::with_capacity(frames);
    let mut r_all = Vec::with_capacity(frames);
    let mut l = [0.0f32; BLOCK];
    let mut r = [0.0f32; BLOCK];
    while l_all.len() < frames {
        dsp.compute(&mut l, &mut r);
        l_all.extend_from_slice(&l);
        r_all.extend_from_slice(&r);
    }
    (l_all, r_all)
}

fn rms(x: &[f32]) -> f32 {
    (x.iter().map(|s| s * s).sum::<f32>() / x.len() as f32).sqrt()
}

#[test]
fn is_stereo() {
    assert_eq!(NUM_OUTPUTS, 2);
}

#[test]
fn exposes_expected_parameters() {
    let dsp = Dsp::new(SR);
    for path in [
        "drone_1/pitch",
        "drone_1/detune",
        "drone_1/sub",
        "drone_1/drift",
        "drone_1/level",
        "drone_6/pitch",
        "drone_6/level",
        "filter/cutoff",
        "filter/resonance",
        "space/reverb_mix",
        "master/gain",
    ] {
        assert!(dsp.param_id(path).is_some(), "missing parameter {path}");
    }
    let cutoff = &dsp.params()[dsp.param_id("filter/cutoff").unwrap()];
    assert_eq!(cutoff.scale, Scale::Log);
    assert_eq!(cutoff.unit.as_deref(), Some("Hz"));
}

#[test]
fn produces_sound_within_full_scale() {
    let mut dsp = Dsp::new(SR);
    let (l, r) = render(&mut dsp, 2.0);
    let tail = l.len() / 2; // skip the reverb build-up
    assert!(rms(&l[tail..]) > 0.01, "left channel is silent");
    assert!(rms(&r[tail..]) > 0.01, "right channel is silent");
    assert!(
        l.iter()
            .chain(r.iter())
            .all(|s| s.is_finite() && s.abs() <= 1.0),
        "output is not finite or exceeds full scale"
    );
}

#[test]
fn closing_the_filter_darkens_the_sound() {
    // Brightness proxy: energy of the first difference relative to the signal.
    fn brightness(x: &[f32]) -> f32 {
        let diff: Vec<f32> = x.windows(2).map(|w| w[1] - w[0]).collect();
        rms(&diff) / rms(x).max(1e-9)
    }

    let mut open = Dsp::new(SR);
    let mut closed = Dsp::new(SR);
    let cutoff = open.param_id("filter/cutoff").unwrap();
    open.set(cutoff, 6000.0);
    closed.set(cutoff, 200.0);

    let (lo, _) = render(&mut open, 1.5);
    let (lc, _) = render(&mut closed, 1.5);
    let n = lo.len();
    assert!(brightness(&lo[n / 2..]) > brightness(&lc[n / 2..]));
}

#[test]
fn set_clamps_to_range_and_ignores_garbage() {
    let mut dsp = Dsp::new(SR);
    let gain = dsp.param_id("master/gain").unwrap();
    dsp.set(gain, 5.0);
    assert_eq!(dsp.get(gain), Some(1.0));
    dsp.set(gain, f32::NAN);
    assert!(dsp.get(gain).unwrap().is_finite());
    dsp.set(9_999, 1.0); // unknown id: no panic
}

#[test]
fn six_voices_together_grow_louder_but_stay_in_range() {
    let mut two = Dsp::new(SR);
    let mut six = Dsp::new(SR);
    for v in 1..=6 {
        let id = six.param_id(&format!("drone_{v}/level")).unwrap();
        six.set(id, 1.0);
    }
    let (l2, _) = render(&mut two, 3.0);
    let (l6, r6) = render(&mut six, 3.0);
    let n = l2.len();
    assert!(
        rms(&l6[n / 2..]) > rms(&l2[n / 2..]),
        "more voices should sound bigger"
    );
    assert!(
        l6.iter()
            .chain(r6.iter())
            .all(|s| s.is_finite() && s.abs() < 0.9),
        "six voices at full level should keep headroom"
    );
}

#[test]
fn voices_are_tuned_to_an_open_minor_chord_and_only_two_sound_at_rest() {
    let dsp = Dsp::new(SR);
    let init = |path: &str| dsp.params()[dsp.param_id(path).unwrap()].init;
    let pitches: Vec<f32> = (1..=6).map(|v| init(&format!("drone_{v}/pitch"))).collect();
    assert_eq!(pitches, vec![55.0, 82.41, 110.0, 130.81, 164.81, 220.0]);
    let sounding = (1..=6)
        .filter(|v| init(&format!("drone_{v}/level")) > 0.0)
        .count();
    assert_eq!(sounding, 2);
}
