//! What the `sequence` tool (and `audition` with a sequence) asks for,
//! turned into a [`Pattern`] for the director's sequencer.

use moodscore_director::harmony::parse_note;
use moodscore_director::sequence::{self, Order, Pattern, Source};
use serde::Deserialize;

use crate::notes;

/// The musical part of a sequence: what it plays and how.
#[derive(Debug, Clone, Default, Deserialize, schemars::JsonSchema)]
pub struct SequenceSpec {
    /// Arpeggiate the chord sounding (from harmony): up, down, up_down, down_up, random (never the same note twice in a row) or chord (all its notes on every step). It starts again from the first note on each new chord. Give arp or motif.
    pub arp: Option<String>,
    /// The arpeggio's register as [low, high], e.g. ["F4", "C#5"]: each chord is spread inside it, so the notes two chords share stay put. Default ["C4", "C5"].
    pub range: Option<(String, String)>,
    /// Arpeggiate these notes instead of the chord sounding, e.g. "A3 C4 E4": no harmony needed, and they stay put when the harmony moves.
    pub notes: Option<String>,
    /// A motif, one token per step. Degrees follow the harmony: "1 5 8 5 3 5 b7 5" (1 is the chord's root, 3 5 7 its own third, fifth and seventh, other degrees from the key; b or # alters from the major scale, 8 is the octave, <5 is an octave down, >3 an octave up). Note names stay put while the harmony moves: "F4 r C5 A#4" (upper case: b3 is a degree, B3 a note). r rest, - hold the previous note, ~ after a note ties it into the next (a legato patch slides), @0.9 velocity, + for several at once ("1+5"). Up to 64 steps.
    pub motif: Option<String>,
    /// Where a motif's degrees sit: each chord's root goes as close to this note as it can, e.g. "A#1" for a bass line. Default "C4".
    pub around: Option<String>,
    /// Step length: "1/16", "1/8", "1/8t" (triplet), "1/4d" (dotted), "1/4"… Follows the tempo. Default "1/16".
    pub rate: Option<String>,
    /// The arpeggio's rhythm, one character per step, repeating: x play, X accent, . rest, - hold the previous note. E.g. "x.xX .x-x". Default "x" (every step). A motif carries its own rhythm.
    pub rhythm: Option<String>,
    /// Fraction of each note held, 0.05 to 1. Default 0.8.
    pub gate: Option<f32>,
    /// Tie every note into the next, so a legato patch (glide_arp, moog_bass) slides between them. Default false.
    pub legato: Option<bool>,
    /// 0 to 1. Default 0.8.
    pub velocity: Option<f32>,
    /// Added to the velocity of accented steps (X), 0 to 0.5. Default 0.15.
    pub accent: Option<f32>,
    /// 0 to just under 0.5: every second step lands this fraction of a step late (0.33 is a shuffle). Default 0.
    pub swing: Option<f32>,
}

fn note(s: &str, what: &str) -> Result<u8, String> {
    parse_note(s)
        .map_err(|e| format!("{what}: {e}"))?
        .midi
        .ok_or_else(|| format!("{what}: give the octave too, e.g. \"{s}4\""))
}

fn given(s: &Option<String>) -> Option<&str> {
    s.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

impl SequenceSpec {
    /// The pattern, and what it plays in words.
    pub fn pattern(&self) -> Result<(Pattern, String), String> {
        let rate_name = given(&self.rate).unwrap_or("1/16");
        let rate = sequence::parse_rate(rate_name)?;
        let gate = self.gate.unwrap_or(0.8);
        if !gate.is_finite() || !(0.05..=1.0).contains(&gate) {
            return Err("gate must be between 0.05 and 1 (use legato to tie notes)".into());
        }
        let velocity = self.velocity.unwrap_or(0.8);
        if !velocity.is_finite() || !(0.0..=1.0).contains(&velocity) {
            return Err("velocity must be between 0 and 1".into());
        }
        let accent = self.accent.unwrap_or(0.15);
        if !accent.is_finite() || !(0.0..=0.5).contains(&accent) {
            return Err("accent must be between 0 and 0.5".into());
        }
        let swing = self.swing.unwrap_or(0.0);
        if !swing.is_finite() || !(0.0..0.5).contains(&swing) {
            return Err("swing must be from 0 to just under 0.5".into());
        }
        let legato = self.legato.unwrap_or(false);
        let mut how = format!("in {rate_name}");
        if legato {
            how.push_str(", legato");
        }
        if swing > 0.0 {
            how.push_str(&format!(", swing {swing}"));
        }

        let (source, steps, plays) = match (given(&self.arp), given(&self.motif)) {
            (Some(_), Some(_)) => return Err("give arp or motif, not both".into()),
            (None, None) => {
                return Err(
                    "sequence needs arp (e.g. \"up\") or motif (e.g. \"1 5 8 5\"), or stop: true"
                        .into(),
                )
            }
            (Some(order), None) => {
                if self.around.is_some() {
                    return Err("around is for a motif; an arpeggio takes range".into());
                }
                let order: Order = order.parse()?;
                let (lo_name, hi_name) = self
                    .range
                    .clone()
                    .unwrap_or_else(|| ("C4".into(), "C5".into()));
                let (low, high) = (note(&lo_name, "range")?, note(&hi_name, "range")?);
                if high <= low || high - low > 48 {
                    return Err(
                        "range: the high note must be above the low one, at most four octaves"
                            .into(),
                    );
                }
                let fixed = match given(&self.notes) {
                    Some(n) => {
                        let mut v = notes::parse_notes(n).map_err(|e| format!("notes: {e}"))?;
                        v.sort_unstable();
                        v.dedup();
                        if v.is_empty() || v.len() > 16 {
                            return Err("notes: give 1 to 16 notes, e.g. \"A3 C4 E4\"".into());
                        }
                        Some(v)
                    }
                    None => None,
                };
                let steps = sequence::parse_rhythm(given(&self.rhythm).unwrap_or("x"))?;
                let over = match (&fixed, &self.range) {
                    (Some(_), _) => format!("over {}", given(&self.notes).unwrap_or("")),
                    (None, _) => format!("over the chord, {lo_name}-{hi_name}"),
                };
                let rhythm = given(&self.rhythm)
                    .filter(|r| *r != "x")
                    .map(|r| format!(", rhythm \"{r}\""))
                    .unwrap_or_default();
                (
                    Source::Arp {
                        order,
                        low,
                        high,
                        notes: fixed,
                    },
                    steps,
                    format!("arpeggio {order} {over}, {how}{rhythm}"),
                )
            }
            (None, Some(m)) => {
                if self.rhythm.is_some() {
                    return Err(
                        "a motif carries its own rhythm: r for a rest, - to hold the previous note"
                            .into(),
                    );
                }
                if self.range.is_some() || self.notes.is_some() {
                    return Err("range and notes are for an arpeggio; a motif takes around".into());
                }
                let around_name = given(&self.around).unwrap_or("C4");
                let around = note(around_name, "around")?;
                let steps = sequence::parse_motif(m)?;
                let follows = steps.iter().any(|s| {
                    s.pitches
                        .iter()
                        .any(|p| matches!(p, sequence::Pitch::Degree { .. }))
                });
                let plays = if follows {
                    format!("motif \"{m}\" around {around_name}, following the harmony, {how}")
                } else {
                    format!("motif \"{m}\", fixed notes, {how}")
                };
                (Source::Motif { around }, steps, plays)
            }
        };
        Ok((
            Pattern {
                source,
                steps,
                rate,
                gate,
                velocity,
                accent,
                swing,
                legato,
            },
            plays,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> SequenceSpec {
        SequenceSpec::default()
    }

    #[test]
    fn arpeggios_and_motifs_build() {
        let (p, plays) = SequenceSpec {
            arp: Some("up_down".into()),
            range: Some(("F4".into(), "C#5".into())),
            rate: Some("1/8t".into()),
            legato: Some(true),
            ..spec()
        }
        .pattern()
        .unwrap();
        assert!(p.needs_harmony());
        assert!(p.legato);
        assert_eq!(
            plays,
            "arpeggio up_down over the chord, F4-C#5, in 1/8t, legato"
        );
        let (p, plays) = SequenceSpec {
            motif: Some("1 5 8 5".into()),
            around: Some("A#1".into()),
            rate: Some("1/8".into()),
            ..spec()
        }
        .pattern()
        .unwrap();
        assert!(p.needs_harmony());
        assert_eq!(p.rate, 0.5);
        assert!(plays.contains("following the harmony"), "{plays}");
        let (p, _) = SequenceSpec {
            arp: Some("random".into()),
            notes: Some("A3 C4 E4".into()),
            ..spec()
        }
        .pattern()
        .unwrap();
        assert!(!p.needs_harmony());
    }

    #[test]
    fn mistakes_are_explained() {
        let bad = [
            SequenceSpec::default(),
            SequenceSpec {
                arp: Some("up".into()),
                motif: Some("1".into()),
                ..spec()
            },
            SequenceSpec {
                arp: Some("sideways".into()),
                ..spec()
            },
            SequenceSpec {
                motif: Some("1 5".into()),
                rhythm: Some("x.".into()),
                ..spec()
            },
            SequenceSpec {
                arp: Some("up".into()),
                range: Some(("C5".into(), "C4".into())),
                ..spec()
            },
            SequenceSpec {
                arp: Some("up".into()),
                swing: Some(0.5),
                ..spec()
            },
            SequenceSpec {
                arp: Some("up".into()),
                rate: Some("fast".into()),
                ..spec()
            },
        ];
        for b in bad {
            assert!(b.pattern().is_err(), "{b:?}");
        }
    }
}
