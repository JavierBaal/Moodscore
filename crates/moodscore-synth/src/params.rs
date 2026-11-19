//! Every patch parameter, in one table.
//!
//! A patch is a flat array of `f32`, one per entry in [`PARAMS`], indexed by
//! [`P`]. That keeps it trivially copyable across threads, lets the engine
//! smooth it like any other parameter, and lets presets be plain
//! `path → value` maps.
//!
//! Continuous parameters can move while a note sounds. Discrete ones (counts,
//! choices) are read when a note starts.

/// How a parameter's range is meant to be traversed.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Linear,
    /// Geometric (frequencies, times).
    Log,
    /// Whole numbers between min and max.
    Int,
    /// One of a list of named choices, stored as its index.
    Choice(&'static [&'static str]),
}

impl Kind {
    pub fn is_discrete(self) -> bool {
        matches!(self, Kind::Int | Kind::Choice(_))
    }
}

/// One patch parameter.
#[derive(Copy, Clone, Debug)]
pub struct ParamDef {
    /// Stable path inside a part, e.g. `"filter/cutoff"`.
    pub path: &'static str,
    pub min: f32,
    pub max: f32,
    pub init: f32,
    pub kind: Kind,
    pub unit: Option<&'static str>,
    /// What it does, in a few words, for the agent.
    pub description: &'static str,
}

impl ParamDef {
    /// Clamp and, for discrete parameters, round to a valid value.
    pub fn sanitize(&self, v: f32) -> f32 {
        let v = if v.is_finite() { v } else { self.init };
        let v = v.clamp(self.min, self.max);
        if self.kind.is_discrete() {
            v.round()
        } else {
            v
        }
    }

    /// Position in the range, 0..1 (geometric for `Log`).
    pub fn normalize(&self, v: f32) -> f32 {
        match self.kind {
            Kind::Log => (v.max(self.min) / self.min).ln() / (self.max / self.min).ln(),
            _ => (v - self.min) / (self.max - self.min),
        }
    }

    /// Inverse of [`normalize`](Self::normalize), clamped to the range.
    pub fn denormalize(&self, n: f32) -> f32 {
        let n = n.clamp(0.0, 1.0);
        match self.kind {
            Kind::Log => self.min * (self.max / self.min).powf(n),
            _ => self.min + (self.max - self.min) * n,
        }
    }

    /// The name of a choice value, if this is a choice parameter.
    pub fn choice_name(&self, v: f32) -> Option<&'static str> {
        match self.kind {
            Kind::Choice(names) => names.get(self.sanitize(v) as usize).copied(),
            _ => None,
        }
    }

    /// The index of a named choice.
    pub fn choice_index(&self, name: &str) -> Option<f32> {
        match self.kind {
            Kind::Choice(names) => names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name.trim()))
                .map(|i| i as f32),
            _ => None,
        }
    }
}

pub const FILTER_TYPES: &[&str] = &["low_pass", "band_pass", "high_pass"];
pub const LFO_SHAPES: &[&str] = &["sine", "triangle", "saw", "square", "random"];
/// Tempo divisions for synced LFOs, in beats.
pub const DIVISIONS: &[&str] = &[
    "1/32", "1/16", "1/16t", "1/8", "1/8t", "1/8d", "1/4", "1/4t", "1/4d", "1/2", "1/2d", "1", "2",
    "4",
];
pub const DIVISION_BEATS: &[f32] = &[
    0.125,
    0.25,
    1.0 / 6.0,
    0.5,
    1.0 / 3.0,
    0.75,
    1.0,
    2.0 / 3.0,
    1.5,
    2.0,
    3.0,
    4.0,
    8.0,
    16.0,
];
pub const MOD_SOURCES: &[&str] = &[
    "none",
    "mod_env",
    "filter_env",
    "amp_env",
    "lfo1",
    "lfo2",
    "velocity",
    "note",
    "random",
    "expression",
];

macro_rules! params {
    ($( $id:ident = $path:literal, $min:expr, $max:expr, $init:expr, $kind:expr, $unit:expr, $desc:literal; )*) => {
        /// Parameter ids: the index of each parameter in a patch.
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[repr(usize)]
        pub enum P { $($id),* }

        /// Every patch parameter, in the order of [`P`].
        pub const PARAMS: &[ParamDef] = &[
            $( ParamDef { path: $path, min: $min, max: $max, init: $init, kind: $kind, unit: $unit, description: $desc } ),*
        ];
    };
}

use Kind::*;
const HZ: Option<&str> = Some("Hz");
const S: Option<&str> = Some("s");
const CT: Option<&str> = Some("cents");
const ST: Option<&str> = Some("semitones");
const OCT: Option<&str> = Some("octaves");

params! {
    Osc1Level = "osc1/level", 0.0, 1.0, 0.8, Linear, None, "level of oscillator 1";
    Osc1Shape = "osc1/shape", 0.0, 3.0, 2.0, Linear, None, "waveform, morphing: 0 sine, 1 triangle, 2 saw, 3 square";
    Osc1Pw = "osc1/pw", 0.05, 0.95, 0.5, Linear, None, "pulse width of the square part";
    Osc1Octave = "osc1/octave", -3.0, 3.0, 0.0, Int, None, "octave offset";
    Osc1Semi = "osc1/semi", -12.0, 12.0, 0.0, Int, ST, "semitone offset";
    Osc1Fine = "osc1/fine", -100.0, 100.0, 0.0, Linear, CT, "fine tuning";
    Osc1Unison = "osc1/unison", 1.0, 7.0, 1.0, Int, None, "number of detuned copies";
    Osc1Detune = "osc1/detune", 0.0, 100.0, 15.0, Linear, CT, "spread of the unison copies' tuning";
    Osc1Spread = "osc1/spread", 0.0, 1.0, 0.6, Linear, None, "stereo width of the unison copies";
    Osc1Sync = "osc1/sync", 1.0, 8.0, 1.0, Log, None, "hard sync: runs this many times faster, restarting every cycle (1 = off)";
    Osc1Phase = "osc1/random_phase", 0.0, 1.0, 1.0, Linear, None, "0 every note starts at the same phase (punchy), 1 free (analog)";

    Osc2Level = "osc2/level", 0.0, 1.0, 0.0, Linear, None, "level of oscillator 2";
    Osc2Shape = "osc2/shape", 0.0, 3.0, 2.0, Linear, None, "waveform, morphing: 0 sine, 1 triangle, 2 saw, 3 square";
    Osc2Pw = "osc2/pw", 0.05, 0.95, 0.5, Linear, None, "pulse width of the square part";
    Osc2Octave = "osc2/octave", -3.0, 3.0, 0.0, Int, None, "octave offset";
    Osc2Semi = "osc2/semi", -12.0, 12.0, 0.0, Int, ST, "semitone offset";
    Osc2Fine = "osc2/fine", -100.0, 100.0, 7.0, Linear, CT, "fine tuning";
    Osc2Unison = "osc2/unison", 1.0, 7.0, 1.0, Int, None, "number of detuned copies";
    Osc2Detune = "osc2/detune", 0.0, 100.0, 15.0, Linear, CT, "spread of the unison copies' tuning";
    Osc2Spread = "osc2/spread", 0.0, 1.0, 0.6, Linear, None, "stereo width of the unison copies";
    Osc2Sync = "osc2/sync", 1.0, 8.0, 1.0, Log, None, "hard sync ratio (1 = off)";
    Osc2Phase = "osc2/random_phase", 0.0, 1.0, 1.0, Linear, None, "0 same start phase every note, 1 free";

    FmAmount = "fm/amount", 0.0, 4.0, 0.0, Linear, None, "oscillator 2 modulates oscillator 1's frequency (bells, glass, metal)";
    SubLevel = "sub/level", 0.0, 1.0, 0.0, Linear, None, "sine sub-oscillator";
    SubOctave = "sub/octave", -2.0, -1.0, -1.0, Int, OCT, "sub-oscillator octave below the note";
    NoiseLevel = "noise/level", 0.0, 1.0, 0.0, Linear, None, "white noise, through the filter";

    Fold = "shape/fold", 0.0, 1.0, 0.0, Linear, None, "wavefolding: adds bright, hollow harmonics";
    Drive = "shape/drive", 0.0, 1.0, 0.0, Linear, None, "saturation before the filter";

    FilterType = "filter/type", 0.0, 2.0, 0.0, Choice(FILTER_TYPES), None, "low_pass, band_pass or high_pass";
    FilterSlope = "filter/slope", 12.0, 24.0, 12.0, Int, Some("dB/oct"), "12 (gentle) or 24 (steep)";
    Cutoff = "filter/cutoff", 20.0, 20000.0, 8000.0, Log, HZ, "filter cutoff";
    Resonance = "filter/resonance", 0.0, 1.0, 0.1, Linear, None, "emphasis at the cutoff";
    Keytrack = "filter/keytrack", 0.0, 1.0, 0.5, Linear, None, "how much the cutoff follows the note (1 = fully)";
    FilterEnv = "filter/env_amount", -6.0, 6.0, 0.0, Linear, OCT, "how far the filter envelope moves the cutoff";

    AmpAttack = "amp_env/attack", 0.001, 10.0, 0.005, Log, S, "volume envelope attack";
    AmpDecay = "amp_env/decay", 0.005, 20.0, 0.3, Log, S, "volume envelope decay";
    AmpSustain = "amp_env/sustain", 0.0, 1.0, 0.8, Linear, None, "volume envelope sustain level";
    AmpRelease = "amp_env/release", 0.005, 20.0, 0.3, Log, S, "volume envelope release";
    Velocity = "amp_env/velocity", 0.0, 1.0, 0.5, Linear, None, "how much velocity changes the volume";

    FenvAttack = "filter_env/attack", 0.001, 10.0, 0.005, Log, S, "filter envelope attack";
    FenvDecay = "filter_env/decay", 0.005, 20.0, 0.3, Log, S, "filter envelope decay";
    FenvSustain = "filter_env/sustain", 0.0, 1.0, 0.0, Linear, None, "filter envelope sustain level";
    FenvRelease = "filter_env/release", 0.005, 20.0, 0.3, Log, S, "filter envelope release";

    MenvAttack = "mod_env/attack", 0.001, 10.0, 0.005, Log, S, "modulation envelope attack";
    MenvDecay = "mod_env/decay", 0.005, 20.0, 0.5, Log, S, "modulation envelope decay";
    MenvSustain = "mod_env/sustain", 0.0, 1.0, 0.0, Linear, None, "modulation envelope sustain level";
    MenvRelease = "mod_env/release", 0.005, 20.0, 0.5, Log, S, "modulation envelope release";
}

impl P {
    pub fn def(self) -> &'static ParamDef {
        &PARAMS[self as usize]
    }
}

/// Number of patch parameters.
pub fn count() -> usize {
    PARAMS.len()
}

/// Look up a parameter id by path.
pub fn find(path: &str) -> Option<usize> {
    PARAMS.iter().position(|p| p.path == path)
}

/// A patch: one value per parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct Patch(pub Vec<f32>);

impl Default for Patch {
    fn default() -> Self {
        Self(PARAMS.iter().map(|p| p.init).collect())
    }
}

impl Patch {
    pub fn get(&self, p: P) -> f32 {
        self.0[p as usize]
    }

    pub fn set(&mut self, p: P, v: f32) {
        self.0[p as usize] = p.def().sanitize(v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_consistent() {
        let mut paths = std::collections::HashSet::new();
        for p in PARAMS {
            assert!(paths.insert(p.path), "duplicate {}", p.path);
            assert!(p.min < p.max, "{}", p.path);
            assert!((p.min..=p.max).contains(&p.init), "{} init", p.path);
            if p.kind == Kind::Log {
                assert!(p.min > 0.0, "{} log needs min > 0", p.path);
            }
            if let Kind::Choice(names) = p.kind {
                assert_eq!(names.len() as f32, p.max - p.min + 1.0, "{}", p.path);
            }
        }
        assert_eq!(DIVISIONS.len(), DIVISION_BEATS.len());
        assert_eq!(P::Osc1Level.def().path, "osc1/level");
        assert_eq!(find("filter/cutoff"), Some(P::Cutoff as usize));
    }

    #[test]
    fn normalize_round_trips() {
        for p in PARAMS {
            for v in [p.min, p.init, p.max] {
                let back = p.denormalize(p.normalize(v));
                assert!(
                    (back - v).abs() <= 1e-3 * v.abs().max(1.0),
                    "{} {v}",
                    p.path
                );
            }
        }
        let c = P::FilterType.def();
        assert_eq!(c.choice_index("High_Pass"), Some(2.0));
        assert_eq!(c.choice_name(1.2), Some("band_pass"));
    }
}
