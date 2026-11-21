//! Step sequences: arpeggios and ostinatos that follow the harmony and the
//! tempo.
//!
//! Pure, no audio and no clock. A [`Pattern`] says what to play: a cycle of
//! steps, each one `rate` long, with an arpeggio over the chord that is
//! sounding or a motif. A [`Runner`] turns it into note events at exact
//! times, a little ahead, asking which chord sounds when each step lands, so
//! a chord change or a tempo ramp is followed on the step. The conductor
//! feeds the events to the engine's note agenda; `audition` renders them
//! offline.
//!
//! Motifs are written in degrees or in note names. Degrees follow the
//! harmony: they count up the scale from the chord's root, and the third,
//! fifth and seventh are the chord's own (a borrowed major V brings its
//! leading tone). On a chord with a bass note (`IV/C#`), 1 is that bass and
//! 3, 5 and 7 are the chord's tones going up from it, so a motif plays the
//! inversion: `1+3+5` over A/C# is C# E A. Note names stay put while the
//! harmony moves under them.

use std::fmt;
use std::str::FromStr;

use moodscore_engine::NoteEvent;

use crate::harmony::{parse_note, Chord, Key};
use crate::tempo::TempoMap;

/// Most steps in a pattern.
pub const MAX_STEPS: usize = 64;

/// The chord sounding at some moment, as the sequencer sees it.
#[derive(Clone, Debug, PartialEq)]
pub struct ChordAt {
    pub chord: Chord,
    pub key: Key,
    /// Changes whenever a chord lands, even the same chord again.
    pub id: u64,
}

/// The order an arpeggio walks the chord in.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Order {
    Up,
    Down,
    /// Up then down, without repeating the top and bottom notes.
    UpDown,
    DownUp,
    /// Any chord tone, never the same one twice in a row.
    Random,
    /// Every chord tone at once, on every step.
    Chord,
}

impl FromStr for Order {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s
            .trim()
            .to_ascii_lowercase()
            .replace(['-', ' '], "_")
            .as_str()
        {
            "up" => Ok(Order::Up),
            "down" => Ok(Order::Down),
            "up_down" | "updown" => Ok(Order::UpDown),
            "down_up" | "downup" => Ok(Order::DownUp),
            "random" => Ok(Order::Random),
            "chord" => Ok(Order::Chord),
            other => Err(format!(
                "arp '{other}': use up, down, up_down, down_up, random or chord"
            )),
        }
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Order::Up => "up",
            Order::Down => "down",
            Order::UpDown => "up_down",
            Order::DownUp => "down_up",
            Order::Random => "random",
            Order::Chord => "chord",
        })
    }
}

/// A note of a motif.
#[derive(Clone, Debug, PartialEq)]
pub enum Pitch {
    /// A fixed MIDI note.
    Note(u8),
    /// A degree counted up the scale from the chord's root (1 = the root,
    /// 8 = the root an octave up), with an optional flat or sharp and
    /// octave shift.
    Degree {
        degree: u8,
        accidental: i8,
        octave: i8,
    },
}

/// What a step does.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Hit {
    Play,
    /// Play, louder.
    Accent,
    Rest,
    /// Keep the previous note sounding through this step.
    Hold,
}

/// One step of a pattern.
#[derive(Clone, Debug, PartialEq)]
pub struct Step {
    pub hit: Hit,
    /// The motif's notes for this step (empty for an arpeggio).
    pub pitches: Vec<Pitch>,
    /// This step's velocity, if it has its own.
    pub velocity: Option<f32>,
    /// Hold into the next note (a legato patch slides to it).
    pub tie: bool,
}

/// Where the notes come from.
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    /// Arpeggiate the chord sounding (or `notes`, if given) between `low`
    /// and `high`.
    Arp {
        order: Order,
        low: u8,
        high: u8,
        notes: Option<Vec<u8>>,
    },
    /// Play the steps' own notes. Degrees place the chord's root as close
    /// to `around` as it goes.
    Motif { around: u8 },
}

/// What to play: a cycle of steps.
#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub source: Source,
    pub steps: Vec<Step>,
    /// Quarter notes per step.
    pub rate: f64,
    /// Fraction of a note's length it is held.
    pub gate: f32,
    pub velocity: f32,
    /// Added to the velocity of an accented step.
    pub accent: f32,
    /// 0 to 0.5: every second step lands this fraction of a step late.
    pub swing: f32,
    /// Tie every note into the next.
    pub legato: bool,
}

impl Pattern {
    /// Whether any step needs a chord to know its notes.
    pub fn needs_harmony(&self) -> bool {
        match &self.source {
            Source::Arp { notes, .. } => notes.is_none(),
            Source::Motif { .. } => self
                .steps
                .iter()
                .any(|s| s.pitches.iter().any(|p| matches!(p, Pitch::Degree { .. }))),
        }
    }

    /// Quarter notes one cycle of the pattern lasts.
    pub fn cycle_quarters(&self) -> f64 {
        self.rate * self.steps.len() as f64
    }
}

/// A step length: `1/16`, `1/8t` (triplet), `1/4d` (dotted), `1/2`, `1`,
/// in quarter notes.
pub fn parse_rate(s: &str) -> Result<f64, String> {
    let t = s.trim().to_ascii_lowercase();
    let bad =
        || format!("rate '{s}': use a note value like 1/16, 1/8, 1/8t (triplet) or 1/4d (dotted)");
    let (body, mult) = if let Some(b) = t.strip_suffix('t') {
        (b, 2.0 / 3.0)
    } else if let Some(b) = t.strip_suffix('d') {
        (b, 1.5)
    } else {
        (t.as_str(), 1.0)
    };
    let whole = match body.split_once('/') {
        Some((a, b)) => {
            let a: f64 = a.trim().parse().map_err(|_| bad())?;
            let b: f64 = b.trim().parse().map_err(|_| bad())?;
            if b <= 0.0 {
                return Err(bad());
            }
            a / b
        }
        None => body.trim().parse().map_err(|_| bad())?,
    };
    let q = whole * 4.0 * mult;
    if !q.is_finite() || !(1.0 / 16.0 - 1e-9..=8.0).contains(&q) {
        return Err(format!("rate '{s}': between 1/64 and two whole notes"));
    }
    Ok(q)
}

/// An arpeggio's rhythm: `x` play, `X` accent, `.` rest, `-` hold the
/// previous note; spaces and `|` are ignored. `"x.xX .x-x"`.
pub fn parse_rhythm(s: &str) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    for c in s.chars() {
        let hit = match c {
            'x' | 'o' => Hit::Play,
            'X' | 'O' => Hit::Accent,
            '.' | 'r' | '_' => Hit::Rest,
            '-' => Hit::Hold,
            ' ' | '|' => continue,
            other => {
                return Err(format!(
                    "rhythm '{s}': '{other}' is not a step (x play, X accent, . rest, - hold)"
                ))
            }
        };
        steps.push(Step {
            hit,
            pitches: Vec::new(),
            velocity: None,
            tie: false,
        });
    }
    check_len(steps.len(), "rhythm")?;
    Ok(steps)
}

fn check_len(n: usize, what: &str) -> Result<(), String> {
    if n == 0 {
        return Err(format!("the {what} has no steps"));
    }
    if n > MAX_STEPS {
        return Err(format!("a {what} can have at most {MAX_STEPS} steps"));
    }
    Ok(())
}

/// A motif, one token per step: degrees (`1`, `5`, `b7`, `#4`, `8`, `<5`
/// an octave down, `>3` an octave up) or note names with octave (`F4`,
/// `A#3`; upper case, so `b3` is a degree and `B3` a note); `+` for
/// several at once (`1+5`); `r` rest; `-` hold the previous note; `~` after
/// a note ties it into the next; `@0.9` its velocity. `|` is ignored.
pub fn parse_motif(s: &str) -> Result<Vec<Step>, String> {
    let mut steps = Vec::new();
    for (i, raw) in s.split_whitespace().enumerate() {
        if raw.chars().all(|c| c == '|') {
            continue;
        }
        let at = |e: String| format!("motif token {} ('{raw}'): {e}", i + 1);
        match raw {
            "r" | "R" | "." | "rest" => {
                steps.push(rest(Hit::Rest));
                continue;
            }
            "-" => {
                steps.push(rest(Hit::Hold));
                continue;
            }
            _ => {}
        }
        let (body, velocity) = match raw.split_once('@') {
            Some((b, v)) => (
                b,
                Some(
                    v.trim_end_matches('~')
                        .parse::<f32>()
                        .ok()
                        .filter(|v| (0.0..=1.0).contains(v))
                        .ok_or_else(|| at(format!("'{v}' is not a velocity (0 to 1)")))?,
                ),
            ),
            None => (raw, None),
        };
        let tie = raw.ends_with('~');
        let body = body.trim_end_matches('~');
        let mut pitches = Vec::new();
        for part in body.split('+').filter(|p| !p.is_empty()) {
            pitches.push(parse_pitch(part).map_err(at)?);
        }
        if pitches.is_empty() {
            return Err(at("no note".into()));
        }
        if pitches.len() > 8 {
            return Err(at("at most 8 notes at once".into()));
        }
        steps.push(Step {
            hit: Hit::Play,
            pitches,
            velocity,
            tie,
        });
    }
    check_len(steps.len(), "motif")?;
    Ok(steps)
}

fn rest(hit: Hit) -> Step {
    Step {
        hit,
        pitches: Vec::new(),
        velocity: None,
        tie: false,
    }
}

fn parse_pitch(s: &str) -> Result<Pitch, String> {
    let first = s.chars().next().unwrap_or(' ');
    let is_degree = first.is_ascii_digit()
        || matches!(first, '<' | '>' | '#')
        || (first == 'b' && s[1..].chars().next().is_some_and(|c| c.is_ascii_digit()));
    if !is_degree {
        let n = parse_note(s).map_err(|e| e.to_string())?;
        return n
            .midi
            .map(Pitch::Note)
            .ok_or_else(|| format!("'{s}': give a note's octave (F4) or a degree (5)"));
    }
    let mut rest = s;
    let mut octave = 0i8;
    while let Some(c) = rest.chars().next() {
        match c {
            '<' => octave -= 1,
            '>' => octave += 1,
            _ => break,
        }
        rest = &rest[1..];
    }
    let mut accidental = 0i8;
    while let Some(c) = rest.chars().next() {
        match c {
            'b' => accidental -= 1,
            '#' => accidental += 1,
            _ => break,
        }
        rest = &rest[1..];
    }
    let degree: u8 = rest
        .parse()
        .ok()
        .filter(|d| (1..=15).contains(d))
        .ok_or_else(|| format!("'{s}' is not a degree (1 to 15, e.g. 5, b7, <5) or a note (F4)"))?;
    if octave.abs() > 3 || accidental.abs() > 2 {
        return Err(format!("'{s}' goes too far"));
    }
    Ok(Pitch::Degree {
        degree,
        accidental,
        octave,
    })
}

/// Semitones above the chord's root of a degree (1 = the root). Without an
/// accidental, the third, fifth and seventh are the chord's own when it has
/// them, and every other degree comes from the key's scale; with one, the
/// degree is counted from the major scale (`b7` is always a minor seventh).
pub fn degree_semitones(chord: &Chord, key: &Key, degree: u8, accidental: i8) -> i32 {
    const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
    let d = degree.max(1) as i32 - 1;
    let (octave, class) = (d / 7, (d % 7) as usize);
    let base = if accidental != 0 {
        MAJOR[class] + accidental as i32
    } else if class == 0 {
        0
    } else {
        // The intervals that can stand for this degree, the plain one first.
        let candidates: &[i32] = match class {
            1 => &[2, 1],
            2 => &[4, 3],
            3 => &[5, 6],
            4 => &[7, 6, 8],
            5 => &[9, 8],
            _ => &[10, 11, 9],
        };
        let has = |c: i32| chord.intervals.iter().any(|&i| (i % 12) as i32 == c);
        let own = match class {
            // The chord's third, fifth and seventh, whatever they are.
            2 | 4 | 6 => candidates.iter().copied().find(|&c| has(c)),
            // Its colours: sus2 or add9, sus4.
            1 => has(2).then_some(2),
            3 => has(5).then_some(5),
            _ => None,
        };
        let in_key = || {
            candidates
                .iter()
                .copied()
                .find(|&c| key.contains(((chord.root as i32 + c).rem_euclid(12)) as u8))
        };
        own.or_else(in_key).unwrap_or(candidates[0])
    };
    base + 12 * octave
}

/// Where a motif degree lands on `chord`: the pitch class it counts from
/// and the semitones above it. On a plain chord that is the root and
/// [`degree_semitones`]. On a chord with a bass note it is the bass, and
/// the chord degrees (1, 3, 5, 7) are the chord's tones stacked up from it
/// (over A/C#: C#, E, A); a degree the stack has not got (a 7 on a triad)
/// and 2, 4 and 6 come from the key above the bass, and an accidental
/// counts from the major scale above it.
pub fn motif_degree(chord: &Chord, key: &Key, degree: u8, accidental: i8) -> (u8, i32) {
    const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
    let Some(bass) = chord.bass else {
        return (chord.root, degree_semitones(chord, key, degree, accidental));
    };
    let d = degree.max(1) as i32 - 1;
    let (octave, class) = (d / 7, (d % 7) as usize);
    // The chord stacked up from its bass: semitones above it, the bass first.
    let mut stack: Vec<i32> = chord
        .intervals
        .iter()
        .map(|&i| (chord.root as i32 + i as i32 - bass as i32).rem_euclid(12))
        .chain([0])
        .collect();
    stack.sort_unstable();
    stack.dedup();
    let base = if accidental != 0 {
        MAJOR[class] + accidental as i32
    } else if class % 2 == 0 && class / 2 < stack.len() {
        stack[class / 2]
    } else {
        let candidates: &[i32] = match class {
            1 => &[2, 1],
            2 => &[4, 3],
            3 => &[5, 6],
            4 => &[7, 6, 8],
            5 => &[9, 8],
            _ => &[10, 11, 9],
        };
        candidates
            .iter()
            .copied()
            .find(|&c| key.contains(((bass as i32 + c).rem_euclid(12)) as u8))
            .unwrap_or(candidates[0])
    };
    (bass, base + 12 * octave)
}

/// Where a motif's degrees count from: the chord's root, as close to
/// `around` as it goes.
fn root_near(root_pc: u8, around: u8) -> i32 {
    let diff = (root_pc as i32 - around as i32).rem_euclid(12);
    let diff = if diff > 6 { diff - 12 } else { diff };
    around as i32 + diff
}

/// Chord tones between `low` and `high`, low to high. If none fits, the
/// first one above `low`.
fn arp_notes(pitch_classes: &[u8], low: u8, high: u8) -> Vec<u8> {
    let has = |n: u8| pitch_classes.contains(&(n % 12));
    let notes: Vec<u8> = (low..=high).filter(|&n| has(n)).collect();
    if notes.is_empty() {
        return (low..=low.saturating_add(12).min(127))
            .find(|&n| has(n))
            .into_iter()
            .collect();
    }
    notes
}

/// Plays a [`Pattern`] from a starting beat: call [`fill`](Self::fill) with
/// a time a little ahead of now, again and again.
#[derive(Clone, Debug)]
pub struct Runner {
    pattern: Pattern,
    /// Quarter-note position of step 0.
    origin: f64,
    /// Quarter-note position where it stops, if it does.
    end: Option<f64>,
    /// The next step to play.
    next: u64,
    /// Arpeggio notes played since the chord changed.
    played: usize,
    chord_id: Option<u64>,
    last_index: Option<usize>,
    rng: u64,
    /// Notes tied into the next one, still sounding.
    tied: Vec<u8>,
    done: bool,
}

/// Fraction of a step two tied notes overlap by, so a legato patch slides.
const OVERLAP: f64 = 0.05;

/// Seconds a chord may land after a step and still be that step's chord.
pub const CHORD_SLACK: f64 = 0.06;

impl Runner {
    /// Start `pattern` with step 0 on quarter note `origin` (see
    /// [`TempoMap::beat_at`]); `seed` drives the random order.
    pub fn new(pattern: Pattern, origin: f64, seed: u64) -> Self {
        Self {
            pattern,
            origin,
            end: None,
            next: 0,
            played: 0,
            chord_id: None,
            last_index: None,
            rng: seed | 1,
            tied: Vec::new(),
            done: false,
        }
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// Quarter-note position of step 0.
    pub fn origin(&self) -> f64 {
        self.origin
    }

    /// Where it stops, if it does.
    pub fn end(&self) -> Option<f64> {
        self.end
    }

    /// Stop at quarter note `end` (or earlier, if it was already due to).
    /// Notes still sounding then are let go there.
    pub fn stop_at(&mut self, end: f64) {
        self.end = Some(self.end.map_or(end, |e| e.min(end)));
    }

    /// True once it has stopped and let go of every note.
    pub fn done(&self) -> bool {
        self.done
    }

    /// Quarter-note position of step `i`, swing included.
    fn beat(&self, i: u64) -> f64 {
        let swing = if i % 2 == 1 {
            self.pattern.swing as f64 * self.pattern.rate
        } else {
            0.0
        };
        self.origin + i as f64 * self.pattern.rate + swing
    }

    fn step(&self, i: u64) -> &Step {
        &self.pattern.steps[(i % self.pattern.steps.len() as u64) as usize]
    }

    fn before_end(&self, beat: f64) -> bool {
        self.end.is_none_or(|e| beat < e - 1e-9)
    }

    fn random(&mut self) -> u64 {
        // xorshift64
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        self.rng
    }

    /// Every note event due before time `until` (seconds on the tempo map's
    /// clock) that has not been produced yet, in time order (a note off
    /// before a note on at the same instant). `chords` says which chord
    /// sounds at a time.
    pub fn fill(
        &mut self,
        until: f64,
        tempo: &TempoMap,
        chords: &dyn Fn(f64) -> Option<ChordAt>,
    ) -> Vec<(f64, NoteEvent)> {
        // (time, 0 = off / 1 = on, event)
        let mut out: Vec<(f64, u8, NoteEvent)> = Vec::new();
        let len = self.pattern.steps.len() as u64;
        // Bounded: a step lasts at least a 64th note.
        for _ in 0..4096 {
            if self.done {
                break;
            }
            let i = self.next;
            let b = self.beat(i);
            if !self.before_end(b) {
                let end = self.end.unwrap_or(b);
                let t = tempo.time_at(end);
                if t >= until {
                    break;
                }
                for note in self.tied.drain(..) {
                    out.push((t, 0, NoteEvent::NoteOff { note }));
                }
                self.done = true;
                break;
            }
            let t = tempo.time_at(b);
            if t >= until {
                break;
            }
            self.next += 1;
            let step = self.step(i).clone();
            if !matches!(step.hit, Hit::Play | Hit::Accent) {
                if step.hit == Hit::Rest {
                    // Nothing follows a tie into a rest (it is never set so).
                    for note in self.tied.drain(..) {
                        out.push((t, 0, NoteEvent::NoteOff { note }));
                    }
                }
                continue;
            }
            // How long it lasts: this step and the holds after it.
            let mut k = 1;
            while k < len && self.step(i + k).hit == Hit::Hold {
                k += 1;
            }
            let b_next = self.beat(i + k);
            let next_plays =
                matches!(self.step(i + k).hit, Hit::Play | Hit::Accent) && self.before_end(b_next);
            let tie = (step.tie || self.pattern.legato) && next_plays;

            // Ask for the chord a hair after the step: a chord that lands a
            // few milliseconds late (a plan started "now", off the bar
            // line) still counts as the chord of this step.
            let look = (tempo.time_at(b + self.pattern.rate * 0.5) - t).min(CHORD_SLACK);
            let notes = self.notes(&step, t + look, chords);
            // Let go of the notes tied into this one: the same note is
            // played again, the others overlap a little so a legato patch
            // slides from them.
            let overlap = tempo.time_at(b + self.pattern.rate * OVERLAP) - t;
            for note in std::mem::take(&mut self.tied) {
                let at = if notes.contains(&note) {
                    t
                } else {
                    t + overlap
                };
                out.push((at, 0, NoteEvent::NoteOff { note }));
            }
            if notes.is_empty() {
                continue;
            }
            let base = step.velocity.unwrap_or(self.pattern.velocity);
            let velocity = if step.hit == Hit::Accent {
                base + self.pattern.accent
            } else {
                base
            }
            .clamp(0.0, 1.0);
            for &note in &notes {
                out.push((t, 1, NoteEvent::NoteOn { note, velocity }));
            }
            if tie {
                self.tied = notes;
            } else {
                let mut off_b = b + (b_next - b) * self.pattern.gate as f64;
                if let Some(e) = self.end {
                    off_b = off_b.min(e);
                }
                let off = tempo.time_at(off_b);
                for note in notes {
                    out.push((off, 0, NoteEvent::NoteOff { note }));
                }
            }
        }
        out.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        out.into_iter().map(|(t, _, e)| (t, e)).collect()
    }

    /// The notes a played step sounds, with the chord sounding at `t`.
    fn notes(&mut self, step: &Step, t: f64, chords: &dyn Fn(f64) -> Option<ChordAt>) -> Vec<u8> {
        let mut notes: Vec<u8> = match self.pattern.source.clone() {
            Source::Motif { around } => {
                let chord = step
                    .pitches
                    .iter()
                    .any(|p| matches!(p, Pitch::Degree { .. }))
                    .then(|| chords(t))
                    .flatten();
                step.pitches
                    .iter()
                    .filter_map(|p| match *p {
                        Pitch::Note(n) => Some(n as i32),
                        Pitch::Degree {
                            degree,
                            accidental,
                            octave,
                        } => chord.as_ref().map(|c| {
                            let (from, semitones) =
                                motif_degree(&c.chord, &c.key, degree, accidental);
                            root_near(from, around) + semitones + 12 * octave as i32
                        }),
                    })
                    .filter(|n| (0..=127).contains(n))
                    .map(|n| n as u8)
                    .collect()
            }
            Source::Arp {
                order,
                low,
                high,
                notes,
            } => {
                let pool = match notes {
                    Some(n) => n,
                    None => {
                        let Some(c) = chords(t) else {
                            return Vec::new();
                        };
                        if self.chord_id != Some(c.id) {
                            self.chord_id = Some(c.id);
                            self.played = 0;
                            self.last_index = None;
                        }
                        arp_notes(&c.chord.pitch_classes(), low, high)
                    }
                };
                self.pick(order, &pool)
            }
        };
        notes.sort_unstable();
        notes.dedup();
        notes
    }

    /// The next arpeggio note(s) from `pool` (low to high).
    fn pick(&mut self, order: Order, pool: &[u8]) -> Vec<u8> {
        let n = pool.len();
        if n == 0 {
            return Vec::new();
        }
        let p = self.played;
        self.played += 1;
        let up_down = |p: usize| {
            if n == 1 {
                return 0;
            }
            let cycle = 2 * n - 2;
            let k = p % cycle;
            if k < n {
                k
            } else {
                cycle - k
            }
        };
        let idx = match order {
            Order::Chord => return pool.to_vec(),
            Order::Up => p % n,
            Order::Down => n - 1 - p % n,
            Order::UpDown => up_down(p),
            Order::DownUp => n - 1 - up_down(p),
            Order::Random => {
                let mut i = (self.random() % n as u64) as usize;
                if n > 1 && Some(i) == self.last_index {
                    i = (i + 1 + (self.random() % (n as u64 - 1)) as usize) % n;
                }
                i
            }
        };
        self.last_index = Some(idx);
        vec![pool[idx]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::Curve;

    fn key(t: &str, m: &str) -> Key {
        Key::new(t, m).unwrap()
    }

    fn at(symbol: &str, k: &Key, id: u64) -> ChordAt {
        ChordAt {
            chord: Chord::parse(symbol, k).unwrap(),
            key: k.clone(),
            id,
        }
    }

    fn tempo(bpm: f64) -> TempoMap {
        TempoMap::default().changed(0.0, Some(bpm), None, 0.0, Curve::Linear, true)
    }

    fn arp(order: Order, low: &str, high: &str, rhythm: &str, rate: &str) -> Pattern {
        Pattern {
            source: Source::Arp {
                order,
                low: parse_note(low).unwrap().midi.unwrap(),
                high: parse_note(high).unwrap().midi.unwrap(),
                notes: None,
            },
            steps: parse_rhythm(rhythm).unwrap(),
            rate: parse_rate(rate).unwrap(),
            gate: 0.5,
            velocity: 0.8,
            accent: 0.15,
            swing: 0.0,
            legato: false,
        }
    }

    fn motif(m: &str, around: &str) -> Pattern {
        Pattern {
            source: Source::Motif {
                around: parse_note(around).unwrap().midi.unwrap(),
            },
            steps: parse_motif(m).unwrap(),
            rate: 0.5,
            gate: 0.5,
            velocity: 0.8,
            accent: 0.15,
            swing: 0.0,
            legato: false,
        }
    }

    /// (time, note) of every note on.
    fn ons(ev: &[(f64, NoteEvent)]) -> Vec<(f64, u8)> {
        ev.iter()
            .filter_map(|(t, e)| match e {
                NoteEvent::NoteOn { note, .. } => Some((*t, *note)),
                _ => None,
            })
            .collect()
    }

    fn names(ev: &[(f64, NoteEvent)]) -> Vec<u8> {
        ons(ev).into_iter().map(|(_, n)| n).collect()
    }

    #[test]
    fn rates_parse() {
        assert_eq!(parse_rate("1/16").unwrap(), 0.25);
        assert!((parse_rate("1/8t").unwrap() - 1.0 / 3.0).abs() < 1e-12);
        assert_eq!(parse_rate("1/4d").unwrap(), 1.5);
        assert_eq!(parse_rate("1").unwrap(), 4.0);
        for bad in ["", "1/0", "x", "1/128", "4"] {
            assert!(parse_rate(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn steps_land_on_the_grid_and_follow_a_ramp() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        // 120 bpm: a sixteenth every 0.125 s.
        let mut r = Runner::new(arp(Order::Up, "A3", "A4", "x", "1/16"), 0.0, 1);
        let ev = r.fill(1.0, &tempo(120.0), &chords);
        let times: Vec<f64> = ons(&ev).iter().map(|o| o.0).collect();
        assert_eq!(times.len(), 8);
        for (i, t) in times.iter().enumerate() {
            assert!((t - i as f64 * 0.125).abs() < 1e-9, "{times:?}");
        }
        // Under an accelerando, each step is where the tempo map puts it.
        let ramp = tempo(120.0).changed(0.0, Some(180.0), None, 4.0, Curve::Linear, false);
        let mut r = Runner::new(arp(Order::Up, "A3", "A4", "x", "1/16"), 0.0, 1);
        let ev = r.fill(3.0, &ramp, &chords);
        for (i, (t, _)) in ons(&ev).iter().enumerate() {
            assert!((ramp.beat_at(*t) - i as f64 * 0.25).abs() < 1e-6);
        }
    }

    #[test]
    fn filling_in_pieces_is_the_same_as_at_once() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        let t = tempo(128.0);
        let mut p = arp(Order::UpDown, "A3", "E5", "xX.x -x.x", "1/16");
        p.legato = true;
        let mut whole = Runner::new(p.clone(), 0.0, 7);
        let all = whole.fill(8.0, &t, &chords);
        let mut pieces = Runner::new(p, 0.0, 7);
        let mut parts = Vec::new();
        let mut until: f64 = 0.0;
        while until < 8.0 {
            until += 0.013;
            parts.extend(pieces.fill(until.min(8.0), &t, &chords));
        }
        let ons_all = ons(&all);
        assert_eq!(ons_all, ons(&parts));
        assert_eq!(all.len(), parts.len());
    }

    #[test]
    fn arpeggio_orders() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        let t = tempo(120.0);
        let run = |o: Order| {
            let mut r = Runner::new(arp(o, "A3", "A4", "x", "1/16"), 0.0, 3);
            names(&r.fill(1.99, &t, &chords))
        };
        // A minor between A3 and A4: A3 C4 E4 A4.
        assert_eq!(run(Order::Up)[..8], [57, 60, 64, 69, 57, 60, 64, 69]);
        assert_eq!(run(Order::Down)[..4], [69, 64, 60, 57]);
        assert_eq!(run(Order::UpDown)[..8], [57, 60, 64, 69, 64, 60, 57, 60]);
        assert_eq!(run(Order::DownUp)[..6], [69, 64, 60, 57, 60, 64]);
        let random = run(Order::Random);
        assert!(random.windows(2).all(|w| w[0] != w[1]), "{random:?}");
        assert!(random.iter().all(|n| [57, 60, 64, 69].contains(n)));
        let mut r = Runner::new(arp(Order::Chord, "A3", "A4", "x", "1/4"), 0.0, 3);
        assert_eq!(names(&r.fill(0.1, &t, &chords)), [57, 60, 64, 69]);
    }

    #[test]
    fn the_arpeggio_follows_the_chords_in_its_register() {
        // B-flat minor: i, VI, iv, V, a bar each at 128 bpm.
        let k = key("A#", "aeolian");
        let t = tempo(128.0);
        let bar = t.seconds_per_bar(0.0);
        let prog = ["i", "VI", "iv", "V"];
        let chords = |time: f64| {
            let i = ((time / bar).floor() as usize).min(3);
            Some(at(prog[i], &k, i as u64))
        };
        let mut r = Runner::new(arp(Order::Up, "F4", "C#5", "x", "1/8t"), 0.0, 1);
        let ev = r.fill(4.0 * bar - 0.01, &t, &chords);
        let n = names(&ev);
        // Twelve triplets a bar; the arpeggio restarts on each chord.
        assert_eq!(n.len(), 48);
        assert_eq!(n[..3], [65, 70, 73]); // F4 Bb4 Db5
        assert_eq!(n[12..15], [66, 70, 73]); // Gb4 Bb4 Db5
        assert_eq!(n[24..27], [66, 70, 66]); // Gb4 Bb4 (Eb5 is above the range)
        assert_eq!(n[36..39], [65, 69, 72]); // F4 A4 C5: the borrowed V's leading tone
        assert!(n.iter().all(|&x| (65..=73).contains(&x)));
    }

    #[test]
    fn motifs_play_the_inversion_of_a_chord_with_a_bass_note() {
        let k = key("E", "dorian");
        let t = tempo(120.0);
        // The notes of the first `steps` eighths.
        let notes = |m: &str, around: &str, chord: &str, steps: f64| {
            let c = at(chord, &k, 0);
            let mut r = Runner::new(motif(m, around), 0.0, 1);
            names(&r.fill(steps * 0.25 - 0.01, &t, &|_| Some(c.clone())))
        };
        // A/C#: the chord stacked from its bass (C#3 E3 A3).
        assert_eq!(notes("1+3+5 -", "E3", "IV/C#", 1.0), [49, 52, 57]);
        // A bass line: C#2 A2 C#3 E2.
        assert_eq!(notes("1 5 8 3", "E2", "IV/C#", 4.0), [37, 45, 49, 40]);
        // The same motif over the chord in root position starts on the A
        // nearest E3 (A3 C#4 E4).
        assert_eq!(notes("1+3+5 -", "E3", "IV", 1.0), [57, 61, 64]);
        // 2, 4 and 6 come from the key above the bass; an accidental from
        // the major scale above it.
        let c = Chord::parse("IV/C#", &k).unwrap();
        assert_eq!(motif_degree(&c, &k, 2, 0), (1, 1)); // D, the step above C#
        assert_eq!(motif_degree(&c, &k, 7, 0), (1, 10));
        assert_eq!(motif_degree(&c, &k, 7, -1), (1, 10));
        // A bass outside the chord (D/C) is the 1, and the chord goes on up.
        let d = Chord::parse("bVII/C", &k).unwrap();
        assert_eq!(
            [1, 3, 5, 7].map(|n| motif_degree(&d, &k, n, 0).1),
            [0, 2, 6, 9]
        );
    }

    #[test]
    fn motif_degrees_follow_the_harmony() {
        let k = key("A#", "aeolian");
        let deg =
            |c: &str, d: u8, acc: i8| degree_semitones(&Chord::parse(c, &k).unwrap(), &k, d, acc);
        // i (Bb minor): minor third, perfect fifth, minor seventh from the key.
        assert_eq!([deg("i", 3, 0), deg("i", 5, 0), deg("i", 7, 0)], [3, 7, 10]);
        // V (F major, borrowed): its own major third, the leading tone A.
        assert_eq!(deg("V", 3, 0), 4);
        // VI (Gb major): the second and fourth come from the key (a
        // lydian fourth, C), the third is the chord's.
        assert_eq!(
            [deg("VI", 2, 0), deg("VI", 4, 0), deg("VI", 3, 0)],
            [2, 6, 4]
        );
        // Accidentals count from the major scale; 8 is the octave.
        assert_eq!(
            [
                deg("i", 7, -1),
                deg("i", 4, 1),
                deg("i", 8, 0),
                deg("i", 10, 0)
            ],
            [10, 6, 12, 15]
        );
        // In phrygian, the plain second is the flat one.
        let e = key("E", "phrygian");
        assert_eq!(
            degree_semitones(&Chord::parse("i", &e).unwrap(), &e, 2, 0),
            1
        );

        let t = tempo(120.0);
        let bar = t.seconds_per_bar(0.0);
        let chords = |time: f64| {
            Some(at(
                if time < bar { "i" } else { "V" },
                &k,
                (time >= bar) as u64,
            ))
        };
        let mut r = Runner::new(motif("1 3 5 8 <5 r 5+8 -", "A#3"), 0.0, 1);
        let ev = r.fill(2.0 * bar - 0.01, &t, &chords);
        let n = names(&ev);
        // Over i, around Bb3: Bb3 Db4 F4 Bb4 F3, then F4+Bb4 (a chord).
        assert_eq!(n[..7], [58, 61, 65, 70, 53, 65, 70]);
        // Over V (F), the root F3 is nearest Bb3 below: F3 A3 C4 F4 C3, C4+F4.
        assert_eq!(n[7..14], [53, 57, 60, 65, 48, 60, 65]);
    }

    #[test]
    fn a_chord_a_little_late_still_counts() {
        // The plan started 20 ms after the bar line: its chords change just
        // after each downbeat. The downbeat's step plays the new chord.
        let k = key("A", "aeolian");
        let t = tempo(120.0);
        let chords = |time: f64| {
            let i = ((time - 0.02) / 2.0).floor().max(0.0) as u64;
            Some(at(if i.is_multiple_of(2) { "i" } else { "iv" }, &k, i))
        };
        let mut r = Runner::new(arp(Order::Up, "B3", "B4", "x", "1/4"), 0.0, 1);
        let n = names(&r.fill(3.99, &t, &chords));
        // A minor (C4 E4 A4), then D minor (D4 F4 A4) from the second
        // downbeat on.
        assert_eq!(n, [60, 64, 69, 60, 62, 65, 69, 62]);
    }

    #[test]
    fn fixed_notes_stay_put() {
        let k = key("A", "aeolian");
        let t = tempo(120.0);
        let chords = |time: f64| {
            Some(at(
                if time < 1.0 { "i" } else { "iv" },
                &k,
                (time >= 1.0) as u64,
            ))
        };
        let mut r = Runner::new(motif("A4 r E5 C5", "C4"), 0.0, 1);
        let n = names(&r.fill(3.99, &t, &chords));
        assert_eq!(n, [69, 76, 72, 69, 76, 72, 69, 76, 72, 69, 76, 72]);
    }

    #[test]
    fn rhythm_holds_rests_accents_and_swing() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        let t = tempo(120.0);
        let mut p = arp(Order::Up, "A3", "A4", "xX.x--x.", "1/8");
        p.gate = 1.0;
        let mut r = Runner::new(p.clone(), 0.0, 1);
        let ev = r.fill(1.99, &t, &chords);
        let on: Vec<(f64, u8, f32)> = ev
            .iter()
            .filter_map(|(t, e)| match e {
                NoteEvent::NoteOn { note, velocity } => Some((*t, *note, *velocity)),
                _ => None,
            })
            .collect();
        let times: Vec<f64> = on.iter().map(|o| o.0).collect();
        assert_eq!(times, [0.0, 0.25, 0.75, 1.5]);
        assert!((on[1].2 - 0.95).abs() < 1e-6 && (on[0].2 - 0.8).abs() < 1e-6);
        // The held note lasts three eighths.
        let off_of = |n: u8, after: f64| {
            ev.iter()
                .find(|(t, e)| *t > after && *e == NoteEvent::NoteOff { note: n })
                .map(|(t, _)| *t)
                .unwrap()
        };
        assert!((off_of(on[2].1, 0.75) - 1.5).abs() < 1e-9);
        // Swing pushes every second step late.
        p.swing = 0.25;
        p.steps = parse_rhythm("x").unwrap();
        let mut r = Runner::new(p, 0.0, 1);
        let times: Vec<f64> = ons(&r.fill(1.0, &t, &chords)).iter().map(|o| o.0).collect();
        assert_eq!(times, [0.0, 0.3125, 0.5, 0.8125]);
    }

    #[test]
    fn legato_overlaps_and_ties_into_the_next_note_only() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        let t = tempo(120.0);
        let mut r = Runner::new(motif("A3~ C4~ C4 r E4", "C4"), 0.0, 1);
        let ev = r.fill(1.2, &t, &chords);
        // A3 lets go just after C4 starts (a slide); C4 tied into C4 is
        // played again, its off before the new on; before a rest, no tie.
        let pos =
            |e: NoteEvent, from: f64| ev.iter().position(|(t, x)| *x == e && *t >= from).unwrap();
        let a_off = ev[pos(NoteEvent::NoteOff { note: 57 }, 0.0)].0;
        assert!(a_off > 0.25 && a_off < 0.27, "{a_off}");
        let c_off = pos(NoteEvent::NoteOff { note: 60 }, 0.5);
        let c_on = pos(
            NoteEvent::NoteOn {
                note: 60,
                velocity: 0.8,
            },
            0.5,
        );
        assert!(c_off < c_on && (ev[c_off].0 - 0.5).abs() < 1e-9);
        let last_c_off = ev
            .iter()
            .filter(|(_, e)| *e == NoteEvent::NoteOff { note: 60 })
            .map(|(t, _)| *t)
            .fold(0.0, f64::max);
        assert!((last_c_off - 0.625).abs() < 1e-9, "{last_c_off}");
    }

    #[test]
    fn it_stops_where_asked_and_lets_go() {
        let k = key("A", "aeolian");
        let am = at("i", &k, 1);
        let chords = |_t: f64| Some(am.clone());
        let t = tempo(120.0);
        let mut p = arp(Order::Up, "A3", "A4", "x-", "1/8");
        p.legato = true;
        let mut r = Runner::new(p, 0.0, 1);
        r.stop_at(3.0); // a bar and a half... at 3 quarters
        let ev = r.fill(10.0, &t, &chords);
        assert!(r.done());
        let on = ons(&ev);
        assert!(on.iter().all(|(t, _)| *t < 1.5));
        // Every note that started is released, by the end.
        for (_, n) in &on {
            let offs = ev
                .iter()
                .filter(|(_, e)| *e == NoteEvent::NoteOff { note: *n })
                .count();
            let ons = on.iter().filter(|(_, m)| m == n).count();
            assert_eq!(offs, ons, "note {n}");
        }
        assert!(ev.iter().all(|(t, _)| *t <= 1.5 + 1e-9));
    }

    #[test]
    fn without_a_chord_only_fixed_notes_play() {
        let t = tempo(120.0);
        let none = |_t: f64| None;
        let mut r = Runner::new(motif("1 A4 5", "C4"), 0.0, 1);
        assert_eq!(names(&r.fill(0.7, &t, &none)), [69]);
        let mut p = arp(Order::Up, "A3", "A4", "x", "1/8");
        p.source = Source::Arp {
            order: Order::Up,
            low: 57,
            high: 69,
            notes: Some(vec![57, 60, 64]),
        };
        assert!(!p.needs_harmony());
        let mut r = Runner::new(p, 0.0, 1);
        assert_eq!(names(&r.fill(0.7, &t, &none)), [57, 60, 64]);
    }

    #[test]
    fn motifs_and_rhythms_parse() {
        let m = parse_motif("1 b3@0.9 <5~ >#4 r - F#4+A4 | 8").unwrap();
        assert_eq!(m.len(), 8);
        assert_eq!(
            m[1].pitches,
            [Pitch::Degree {
                degree: 3,
                accidental: -1,
                octave: 0
            }]
        );
        assert_eq!(m[1].velocity, Some(0.9));
        assert!(m[2].tie);
        assert_eq!(m[4].hit, Hit::Rest);
        assert_eq!(m[5].hit, Hit::Hold);
        assert_eq!(m[6].pitches, [Pitch::Note(66), Pitch::Note(69)]);
        assert_eq!(parse_motif("B3").unwrap()[0].pitches, [Pitch::Note(59)]);
        for bad in ["", "16", "0", "H4", "F", "1@2", "x"] {
            assert!(parse_motif(bad).is_err(), "{bad}");
        }
        assert_eq!(parse_rhythm("x.X- |x").unwrap().len(), 5);
        assert!(parse_rhythm("xyz").is_err());
        assert!(parse_rhythm(&"x".repeat(65)).is_err());
    }
}
