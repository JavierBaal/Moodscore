//! Harmony: keys, modes, chords and voice leading.
//!
//! Pure music theory, no time and no audio. The director uses it to turn a
//! harmonic plan ("A aeolian: i, VI, III, VII") into a pitch for each drone
//! voice, moving every voice as little as possible from chord to chord.
//!
//! Chords are written as Roman numerals relative to the key and mode:
//!
//! - The numeral picks the scale degree: `I` … `VII`. Case picks the
//!   quality: upper case is major, lower case is minor. So `v` in aeolian is
//!   the diatonic minor v, and `V` is the borrowed major V.
//! - `b` or `#` before the numeral alters the root: `bVI`, `bVII`, `#iv°`.
//! - `°` (or `o`, `dim`) makes it diminished, `+` (or `aug`) augmented,
//!   `ø` half-diminished (diminished with a minor seventh).
//! - Colour, after the numeral and in any order: `7` (the mode's seventh),
//!   `maj7`, `sus2`, `sus4`, `add9` (the mode's ninth).

use std::fmt;

/// Semitone offsets of each mode's seven degrees from the tonic.
const MODES: [(&str, [u8; 7]); 7] = [
    ("ionian", [0, 2, 4, 5, 7, 9, 11]),
    ("dorian", [0, 2, 3, 5, 7, 9, 10]),
    ("phrygian", [0, 1, 3, 5, 7, 8, 10]),
    ("lydian", [0, 2, 4, 6, 7, 9, 11]),
    ("mixolydian", [0, 2, 4, 5, 7, 9, 10]),
    ("aeolian", [0, 2, 3, 5, 7, 8, 10]),
    ("locrian", [0, 1, 3, 5, 6, 8, 10]),
];

const SHARP_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const FLAT_NAMES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

/// Lowest and highest MIDI notes for the bass voice: E1 .. E2.
pub const BASS_RANGE: (f32, f32) = (28.0, 40.0);
/// Lowest and highest MIDI notes for the upper voices: E2 .. A4.
pub const UPPER_RANGE: (f32, f32) = (40.0, 69.0);

/// A problem with a harmonic plan, explained for the agent.
#[derive(Clone, Debug, PartialEq)]
pub struct HarmonyError(pub String);

impl fmt::Display for HarmonyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for HarmonyError {}

fn err<T>(msg: impl Into<String>) -> Result<T, HarmonyError> {
    Err(HarmonyError(msg.into()))
}

/// MIDI note number (69 = A4 = 440 Hz) to frequency, and back.
pub fn midi_to_hz(m: f32) -> f32 {
    440.0 * 2f32.powf((m - 69.0) / 12.0)
}

pub fn hz_to_midi(hz: f32) -> f32 {
    69.0 + 12.0 * (hz.max(1.0) / 440.0).log2()
}

/// A note name: a pitch class, and an octave if one was given (`E1`).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Note {
    pub pitch_class: u8,
    /// MIDI note number, when the name had an octave.
    pub midi: Option<u8>,
}

/// Parse `A`, `c#`, `Bb`, `E1`, `F#2`. Octaves follow scientific pitch
/// notation (A4 = 440 Hz, C4 = middle C).
pub fn parse_note(s: &str) -> Result<Note, HarmonyError> {
    let t = s.trim();
    let mut chars = t.chars();
    let letter = chars.next().map(|c| c.to_ascii_uppercase());
    let base: i32 = match letter {
        Some('C') => 0,
        Some('D') => 2,
        Some('E') => 4,
        Some('F') => 5,
        Some('G') => 7,
        Some('A') => 9,
        Some('B') => 11,
        _ => return err(format!("'{s}' is not a note (use e.g. A, C#, Bb, E1)")),
    };
    let rest: String = chars.collect();
    let (shift, octave) = match rest.chars().next() {
        Some('#') | Some('♯') => (1, &rest[rest.chars().next().unwrap().len_utf8()..]),
        Some('b') | Some('♭') => (-1, &rest[rest.chars().next().unwrap().len_utf8()..]),
        _ => (0, rest.as_str()),
    };
    let pitch_class = (base + shift).rem_euclid(12) as u8;
    let midi = if octave.is_empty() {
        None
    } else {
        let o: i32 = octave
            .parse()
            .map_err(|_| HarmonyError(format!("'{s}' is not a note (use e.g. A, C#, Bb, E1)")))?;
        let m = (o + 1) * 12 + base + shift;
        if !(0..=127).contains(&m) {
            return err(format!("'{s}' is out of range"));
        }
        Some(m as u8)
    };
    Ok(Note { pitch_class, midi })
}

/// A key: tonic and mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    pub tonic: u8,
    pub mode: String,
    degrees: [u8; 7],
    flats: bool,
    /// Letter of the tonic: 0 = C, 1 = D, … 6 = B.
    letter: u8,
}

impl Key {
    /// `mode` accepts the seven church modes, plus `major` and `minor`.
    pub fn new(tonic: &str, mode: &str) -> Result<Self, HarmonyError> {
        let note = parse_note(tonic)?;
        let m = mode.trim().to_ascii_lowercase();
        let m = match m.as_str() {
            "major" => "ionian",
            "minor" => "aeolian",
            other => other,
        };
        let Some((name, degrees)) = MODES.iter().find(|(n, _)| *n == m) else {
            let names: Vec<&str> = MODES.iter().map(|(n, _)| *n).collect();
            return err(format!(
                "unknown mode '{mode}'; use one of: {}, major, minor",
                names.join(", ")
            ));
        };
        // Spell with flats when the key's relative major is a flat key, or
        // when the tonic was written with a flat.
        let relative_major = (note.pitch_class + 12 - degrees_offset_to_ionian(name)) % 12;
        let flats = [5, 10, 3, 8, 1, 6].contains(&relative_major)
            || tonic.trim().get(1..).is_some_and(|r| r.starts_with('b'));
        let letter = match tonic.trim().chars().next().map(|c| c.to_ascii_uppercase()) {
            Some('C') => 0,
            Some('D') => 1,
            Some('E') => 2,
            Some('F') => 3,
            Some('G') => 4,
            Some('A') => 5,
            _ => 6,
        };
        Ok(Self {
            tonic: note.pitch_class,
            mode: name.to_string(),
            degrees: *degrees,
            flats,
            letter,
        })
    }

    pub fn name(&self, pitch_class: u8) -> &'static str {
        spell(pitch_class, self.flats)
    }

    /// Pitch class of scale degree `d` (0-based).
    fn degree(&self, d: usize) -> u8 {
        (self.tonic + self.degrees[d % 7]) % 12
    }
}

/// Spell `pitch_class` on a given letter (0 = C … 6 = B): `spell_on(8, 5)`
/// is "Ab", `spell_on(8, 4)` is "G#".
fn spell_on(pitch_class: u8, letter: u8) -> String {
    const LETTERS: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];
    const NATURAL: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
    let l = (letter % 7) as usize;
    let diff = (pitch_class as i32 - NATURAL[l] + 18).rem_euclid(12) - 6;
    let acc = match diff {
        -2 => "bb",
        -1 => "b",
        0 => "",
        1 => "#",
        2 => "##",
        // too far from this letter: fall back to a plain spelling
        _ => return spell(pitch_class, diff < 0).to_string(),
    };
    format!("{}{}", LETTERS[l], acc)
}

fn spell(pitch_class: u8, flats: bool) -> &'static str {
    let names = if flats { FLAT_NAMES } else { SHARP_NAMES };
    names[(pitch_class % 12) as usize]
}

/// How far a mode's tonic sits above its relative major's tonic.
fn degrees_offset_to_ionian(mode: &str) -> u8 {
    match mode {
        "dorian" => 2,
        "phrygian" => 4,
        "lydian" => 5,
        "mixolydian" => 7,
        "aeolian" => 9,
        "locrian" => 11,
        _ => 0,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Quality {
    Major,
    Minor,
    Diminished,
    Augmented,
}

/// A chord in a key: its root and the intervals above it, in semitones.
#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    /// As written: `VI`, `iv7`, `bVIImaj7`.
    pub symbol: String,
    /// Readable name in the key: `F`, `Dm7`, `Gmaj7`.
    pub name: String,
    pub root: u8,
    /// A bass note other than the root (`I/C#`): its pitch class.
    pub bass: Option<u8>,
    /// Semitones above the root, root first: `[0, 3, 7]`.
    pub intervals: Vec<u8>,
    /// Note names of the chord tones, spelled by letter.
    spelled: Vec<String>,
}

impl Chord {
    /// Parse a Roman-numeral chord in `key`. See the module docs for syntax.
    pub fn parse(symbol: &str, key: &Key) -> Result<Self, HarmonyError> {
        let help =
            "write chords as Roman numerals: i, VI, bVII, iv7, Vsus4, ii°, IVmaj7, iadd9, I/C#";
        let s = symbol.trim();
        // A slash names the bass note: I/C# is the tonic chord over C#.
        if let Some((chord, bass)) = s.split_once('/') {
            let note = parse_note(bass).map_err(|_| {
                HarmonyError(format!(
                    "'{symbol}': after the slash give the bass note by name (I/C#, IVmaj7/A); \
                     for a secondary dominant write the chord itself (II for V/V)"
                ))
            })?;
            if note.midi.is_some() {
                return err(format!(
                    "'{symbol}': give the bass note without an octave (I/C#)"
                ));
            }
            let mut c = Chord::parse(chord, key)?;
            if note.pitch_class != c.root {
                c.bass = Some(note.pitch_class);
                c.symbol = s.to_string();
                c.name = format!("{}/{}", c.name, key.name(note.pitch_class));
            }
            return Ok(c);
        }
        let mut rest = s;

        let mut accidental: i32 = 0;
        for (p, a) in [("b", -1), ("♭", -1), ("#", 1), ("♯", 1)] {
            if let Some(r) = rest.strip_prefix(p) {
                // "b" alone could not be a numeral, so this is unambiguous.
                accidental = a;
                rest = r;
                break;
            }
        }

        const NUMERALS: [(&str, usize); 7] = [
            ("vii", 6),
            ("iii", 2),
            ("vi", 5),
            ("iv", 3),
            ("ii", 1),
            ("v", 4),
            ("i", 0),
        ];
        let lower = rest.to_ascii_lowercase();
        let Some(&(numeral, degree)) = NUMERALS.iter().find(|(n, _)| lower.starts_with(n)) else {
            return err(format!("'{symbol}' is not a chord; {help}"));
        };
        let written = &rest[..numeral.len()];
        let mut quality = if written.chars().all(|c| c.is_ascii_uppercase()) {
            Quality::Major
        } else if written.chars().all(|c| c.is_ascii_lowercase()) {
            Quality::Minor
        } else {
            return err(format!(
                "'{symbol}': write the numeral all upper case (major) or all lower case (minor)"
            ));
        };
        rest = &rest[numeral.len()..];

        let mut seventh: Option<u8> = None;
        let mut third_override: Option<u8> = None;
        let mut ninth = false;
        let mut suffix = String::new();

        // Quality marks directly after the numeral.
        for (mark, q) in [
            ("dim", Quality::Diminished),
            ("aug", Quality::Augmented),
            ("°", Quality::Diminished),
            ("o", Quality::Diminished),
            ("+", Quality::Augmented),
            ("ø", Quality::Diminished),
        ] {
            if let Some(r) = rest.strip_prefix(mark) {
                quality = q;
                if mark == "ø" {
                    seventh = Some(10);
                }
                rest = r;
                break;
            }
        }

        let root = ((key.degree(degree) as i32 + accidental).rem_euclid(12)) as u8;
        let interval_to = |pc: u8| (pc + 12 - root) % 12;

        while !rest.is_empty() {
            if let Some(r) = rest.strip_prefix("maj7") {
                seventh = Some(11);
                suffix.push_str("maj7");
                rest = r;
            } else if let Some(r) = rest.strip_prefix("sus2") {
                third_override = Some(2);
                suffix.push_str("sus2");
                rest = r;
            } else if let Some(r) = rest.strip_prefix("sus4") {
                third_override = Some(5);
                suffix.push_str("sus4");
                rest = r;
            } else if let Some(r) = rest.strip_prefix("add9") {
                ninth = true;
                suffix.push_str("add9");
                rest = r;
            } else if let Some(r) = rest.strip_prefix('7') {
                seventh = Some(interval_to(key.degree(degree + 6)));
                rest = r;
                suffix.push('7');
            } else {
                return err(format!("'{symbol}': don't understand '{rest}'; {help}"));
            }
        }

        let (third, fifth) = match quality {
            Quality::Major => (4, 7),
            Quality::Minor => (3, 7),
            Quality::Diminished => (3, 6),
            Quality::Augmented => (4, 8),
        };
        // Letter steps above the root for each tone, for spelling.
        let third_steps = match third_override {
            Some(2) => 1,
            Some(5) => 3,
            _ => 2,
        };
        let mut intervals = vec![0, third_override.unwrap_or(third), fifth];
        let mut steps = vec![0u8, third_steps, 4];
        if let Some(s) = seventh {
            intervals.push(s);
            steps.push(6);
        }
        if ninth {
            let n = interval_to(key.degree(degree + 1));
            intervals.push(if n == 0 { 14 } else { n + 12 });
            steps.push(1);
        }

        let root_letter = (key.letter as usize + degree) as u8 % 7;
        let spelled: Vec<String> = intervals
            .iter()
            .zip(&steps)
            .map(|(i, st)| spell_on((root + i % 12) % 12, root_letter + st))
            .collect();
        let name = chord_name(&spelled[0], quality, seventh, &suffix, third_override);
        Ok(Self {
            symbol: s.to_string(),
            name,
            root,
            bass: None,
            intervals,
            spelled,
        })
    }

    /// Pitch classes of the chord tones, root first.
    pub fn pitch_classes(&self) -> Vec<u8> {
        self.intervals
            .iter()
            .map(|i| (self.root + i % 12) % 12)
            .collect()
    }

    /// Note names of the chord tones, root first.
    pub fn note_names(&self) -> Vec<String> {
        self.spelled.clone()
    }
}

fn chord_name(
    root: &str,
    q: Quality,
    seventh: Option<u8>,
    suffix: &str,
    sus: Option<u8>,
) -> String {
    let mut name = root.to_string();
    match (q, seventh) {
        (Quality::Diminished, Some(10)) => name.push_str("m7b5"),
        (Quality::Diminished, Some(9)) => name.push_str("dim7"),
        (Quality::Diminished, _) => name.push_str("dim"),
        (Quality::Augmented, _) => name.push_str("aug"),
        (Quality::Minor, _) if sus.is_none() => name.push('m'),
        _ => {}
    }
    let mut rest = suffix.to_string();
    if q == Quality::Diminished {
        rest = rest.replace('7', "");
    } else if seventh == Some(11) && !rest.contains("maj7") {
        // "7" taken from a mode whose seventh is major: say so.
        rest = rest.replacen('7', "maj7", 1);
    }
    name.push_str(&rest);
    name
}

/// Where each voice should go for a chord.
#[derive(Clone, Debug, PartialEq)]
pub struct Voicing {
    /// MIDI note for the bass voice, if it takes part.
    pub bass: Option<f32>,
    /// MIDI note for each upper voice that takes part, in the order given.
    pub upper: Vec<f32>,
}

/// Voice `chord` for a bass voice (if `bass_from` is given) and the upper
/// voices currently at `upper_from` (MIDI notes), moving as little as
/// possible. `pedal` replaces the chord root in the bass; a chord's own
/// bass note (`I/C#`) wins over the pedal, since it was asked for that chord.
pub fn voice(
    chord: &Chord,
    pedal: Option<Note>,
    bass_from: Option<f32>,
    upper_from: &[f32],
) -> Voicing {
    let bass = bass_from.map(|from| match (chord.bass, pedal) {
        (Some(pc), _) => nearest(pc, from, BASS_RANGE),
        (None, Some(Note { midi: Some(m), .. })) => m as f32,
        (None, Some(Note { pitch_class, .. })) => nearest(pitch_class, from, BASS_RANGE),
        (None, None) => nearest(chord.root, from, BASS_RANGE),
    });

    let upper = assign(chord, upper_from);
    Voicing { bass, upper }
}

/// The chord tones every voicing must include, given how many voices there
/// are. With fewer voices than tones, keep (in order) the root, third,
/// seventh and ninth, dropping the fifth first.
fn required_tones(chord: &Chord, voices: usize) -> Vec<u8> {
    let tones = chord.pitch_classes();
    let mut by_priority: Vec<u8> = Vec::new();
    for idx in [0usize, 1, 3, 4, 2] {
        if let Some(&pc) = tones.get(idx) {
            if !by_priority.contains(&pc) {
                by_priority.push(pc);
            }
        }
    }
    by_priority.truncate(voices);
    by_priority
}

/// The note with pitch class `pc` nearest to `from`, kept inside `range`.
fn nearest(pc: u8, from: f32, range: (f32, f32)) -> f32 {
    let mut best = f32::NAN;
    let mut best_d = f32::INFINITY;
    for octave in -1..=10 {
        let m = (octave * 12 + pc as i32) as f32;
        if m < range.0 || m > range.1 {
            continue;
        }
        let d = (m - from).abs();
        if d < best_d {
            best_d = d;
            best = m;
        }
    }
    best
}

/// Give each voice a chord tone, choosing the combination (and octaves) that
/// moves the voices least while covering the required tones. Doubling the
/// root or fifth is free; doubling the third, seventh or ninth costs a
/// little, and unisons cost more.
fn assign(chord: &Chord, from: &[f32]) -> Vec<f32> {
    let n = from.len();
    if n == 0 {
        return Vec::new();
    }
    let tones = chord.pitch_classes();
    let required = required_tones(chord, n);
    let doubling_cost = |idx: usize| match idx {
        0 | 2 => 0.0, // root, fifth
        _ => 1.5,     // third, seventh, ninth
    };

    let k = tones.len();
    let mut choice = vec![0usize; n];
    let mut best: Option<(f32, Vec<f32>)> = None;
    loop {
        let covered = required
            .iter()
            .all(|pc| choice.iter().any(|&c| tones[c] == *pc));
        if covered {
            let notes: Vec<f32> = choice
                .iter()
                .zip(from)
                .map(|(&c, &f)| nearest(tones[c], f, UPPER_RANGE))
                .collect();
            let mut cost: f32 = notes.iter().zip(from).map(|(a, b)| (a - b).abs()).sum();
            for i in 0..n {
                if choice[..i].contains(&choice[i]) {
                    cost += doubling_cost(choice[i]);
                }
                for j in i + 1..n {
                    if notes[i] == notes[j] {
                        cost += 6.0;
                    }
                }
            }
            if best.as_ref().is_none_or(|(c, _)| cost < *c - 1e-4) {
                best = Some((cost, notes));
            }
        }
        // next combination (odometer over k^n)
        let mut i = 0;
        loop {
            if i == n {
                return best.map(|(_, notes)| notes).unwrap_or_default();
            }
            choice[i] += 1;
            if choice[i] < k {
                break;
            }
            choice[i] = 0;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(t: &str, m: &str) -> Key {
        Key::new(t, m).unwrap()
    }

    fn names(symbol: &str, k: &Key) -> Vec<String> {
        Chord::parse(symbol, k).unwrap().note_names()
    }

    #[test]
    fn notes_parse_with_accidentals_and_octaves() {
        assert_eq!(parse_note("A").unwrap().pitch_class, 9);
        assert_eq!(parse_note("bb").unwrap().pitch_class, 10);
        assert_eq!(parse_note("C#").unwrap().pitch_class, 1);
        assert_eq!(parse_note("A4").unwrap().midi, Some(69));
        assert_eq!(parse_note("E1").unwrap().midi, Some(28));
        assert!(parse_note("H").is_err());
        assert!((midi_to_hz(33.0) - 55.0).abs() < 1e-3);
        assert!((hz_to_midi(82.41) - 40.0).abs() < 0.01);
    }

    #[test]
    fn diatonic_triads_follow_the_mode() {
        let a = key("A", "aeolian");
        assert_eq!(names("i", &a), ["A", "C", "E"]);
        assert_eq!(names("VI", &a), ["F", "A", "C"]);
        assert_eq!(names("III", &a), ["C", "E", "G"]);
        assert_eq!(names("VII", &a), ["G", "B", "D"]);
        assert_eq!(names("ii°", &a), ["B", "D", "F"]);
        let c = key("C", "major");
        assert_eq!(names("V7", &c), ["G", "B", "D", "F"]);
        assert_eq!(names("Imaj7", &c), ["C", "E", "G", "B"]);
        assert_eq!(names("I7", &c), ["C", "E", "G", "B"]); // the mode's seventh
    }

    #[test]
    fn case_borrowing_and_alterations() {
        let a = key("A", "minor");
        assert_eq!(names("V", &a), ["E", "G#", "B"]); // borrowed major V
        assert_eq!(names("v", &a), ["E", "G", "B"]);
        let c = key("C", "major");
        assert_eq!(names("bVI", &c), ["Ab", "C", "Eb"]);
        assert_eq!(names("bVII", &c), ["Bb", "D", "F"]);
        assert_eq!(names("iv", &c), ["F", "Ab", "C"]);
        assert_eq!(names("III+", &c), ["E", "G#", "B#"]);
    }

    #[test]
    fn colour_suffixes() {
        let a = key("A", "aeolian");
        assert_eq!(names("isus2", &a), ["A", "B", "E"]);
        assert_eq!(names("Vsus4", &a), ["E", "A", "B"]);
        assert_eq!(names("iadd9", &a), ["A", "C", "E", "B"]);
        assert_eq!(names("iiø", &a), ["B", "D", "F", "A"]);
        let e = key("E", "phrygian");
        assert_eq!(names("iadd9", &e), ["E", "G", "B", "F"]); // the mode's (flat) ninth
    }

    #[test]
    fn chord_names_read_naturally() {
        let a = key("A", "aeolian");
        let n = |s: &str| Chord::parse(s, &a).unwrap().name;
        assert_eq!(n("i"), "Am");
        assert_eq!(n("VI"), "F");
        assert_eq!(n("iv7"), "Dm7");
        assert_eq!(n("ii°"), "Bdim");
        assert_eq!(n("iiø"), "Bm7b5");
        assert_eq!(n("Vsus4"), "Esus4");
        assert_eq!(n("VImaj7"), "Fmaj7");
        assert_eq!(n("VI7"), "Fmaj7"); // aeolian's seventh above F is E
        assert_eq!(n("VII7"), "G7");
        let d = key("D", "minor");
        assert_eq!(Chord::parse("VI", &d).unwrap().name, "Bb"); // flat key spells with flats
    }

    #[test]
    fn bad_input_is_explained() {
        let a = key("A", "aeolian");
        assert!(Chord::parse("X", &a).unwrap_err().0.contains("Roman"));
        assert!(Chord::parse("Vi", &a).unwrap_err().0.contains("upper case"));
        assert!(Chord::parse("ivzz", &a).unwrap_err().0.contains("zz"));
        assert!(Key::new("A", "spooky").unwrap_err().0.contains("lydian"));
    }

    #[test]
    fn voicing_covers_the_chord_and_moves_little() {
        let a = key("A", "aeolian");
        let from = [40.0, 45.0, 48.0, 52.0, 57.0]; // E2 A2 C3 E3 A3: A minor
        let i = Chord::parse("i", &a).unwrap();
        let v = voice(&i, None, Some(33.0), &from);
        assert_eq!(v.bass, Some(33.0));
        assert_eq!(v.upper, from); // already there: nothing moves

        let vi = Chord::parse("VI", &a).unwrap();
        let v = voice(&vi, None, Some(33.0), &from);
        assert_eq!(v.bass, Some(29.0)); // F1, the nearest F
        let mut pcs: Vec<u8> = v.upper.iter().map(|m| (*m as i32 % 12) as u8).collect();
        pcs.sort();
        pcs.dedup();
        assert_eq!(pcs, vec![0, 5, 9]); // F A C, all present
        let moved: f32 = v.upper.iter().zip(&from).map(|(a, b)| (a - b).abs()).sum();
        assert!(
            moved <= 4.0,
            "upper voices moved {moved} semitones in total"
        );
    }

    #[test]
    fn slash_chords_put_their_note_in_the_bass() {
        let d = Key::new("D", "ionian").unwrap();
        let c = Chord::parse("I/C#", &d).unwrap();
        assert_eq!((c.root, c.bass, c.name.as_str()), (2, Some(1), "D/C#"));
        let c = Chord::parse("IVmaj7/A", &d).unwrap();
        assert_eq!((c.bass, c.name.as_str()), (Some(9), "Gmaj7/A"));
        // Over its own root it is just the chord.
        assert_eq!(Chord::parse("I/D", &d).unwrap().bass, None);
        let e = Chord::parse("V/V", &d).unwrap_err().to_string();
        assert!(e.contains("secondary"), "{e}");
        assert!(Chord::parse("I/C#2", &d).is_err());

        // The bass voice goes to C#, even over a pedal; the upper voices
        // keep the chord.
        let v = voice(
            &Chord::parse("I/C#", &d).unwrap(),
            parse_note("D1").ok(),
            Some(38.0),
            &[50.0, 54.0, 57.0],
        );
        assert_eq!(v.bass.map(|b| b as i32 % 12), Some(1));
        assert!((v.bass.unwrap() - 37.0).abs() < 0.01, "nearest C# to D2");
        let pcs: Vec<i32> = v.upper.iter().map(|m| *m as i32 % 12).collect();
        assert!(pcs.iter().all(|pc| [2, 6, 9].contains(pc)), "{pcs:?}");
    }

    #[test]
    fn a_pedal_holds_the_bass() {
        let a = key("A", "aeolian");
        let vi = Chord::parse("VI", &a).unwrap();
        let v = voice(
            &vi,
            Some(parse_note("A").unwrap()),
            Some(33.0),
            &[45.0, 48.0, 52.0],
        );
        assert_eq!(v.bass, Some(33.0));
        let v = voice(&vi, Some(parse_note("E1").unwrap()), Some(33.0), &[45.0]);
        assert_eq!(v.bass, Some(28.0));
    }

    #[test]
    fn few_voices_keep_the_essential_tones() {
        let a = key("A", "aeolian");
        let ch = Chord::parse("iv7", &a).unwrap(); // D F A C
        let v = voice(&ch, None, None, &[50.0, 53.0]);
        let pcs: Vec<u8> = v.upper.iter().map(|m| (*m as i32 % 12) as u8).collect();
        assert!(
            pcs.contains(&2) && pcs.contains(&5),
            "root and third first: {pcs:?}"
        );
    }

    #[test]
    fn voices_stay_in_range() {
        let a = key("A", "aeolian");
        for sym in ["i", "VI", "III", "VII", "iv7", "Vsus4", "iadd9"] {
            let ch = Chord::parse(sym, &a).unwrap();
            let v = voice(&ch, None, Some(80.0), &[20.0, 100.0, 45.0, 60.0, 69.0]);
            let b = v.bass.unwrap();
            assert!((BASS_RANGE.0..=BASS_RANGE.1).contains(&b), "{sym} bass {b}");
            for m in v.upper {
                assert!(
                    (UPPER_RANGE.0..=UPPER_RANGE.1).contains(&m),
                    "{sym} upper {m}"
                );
            }
        }
    }
}
