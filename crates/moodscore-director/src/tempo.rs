//! Musical time: tempo, metre, and where the music is in bars and beats.
//!
//! The tempo map is analytic: the beat position at any instant is computed
//! in closed form from the last tempo change, never accumulated tick by tick.
//! So it is exact at any sample, whoever asks (the director's control loop
//! today, the audio thread when the step sequencer arrives), and a ramp lands
//! on the beat it should however coarsely it is sampled.
//!
//! Inside, positions are counted in quarter notes, so a change of metre
//! never moves the music. Outside, tempo and beats are in the metre's own
//! beat unit (the lower number): in 4/4 and 3/4 that's the quarter note,
//! in 6/8 the eighth.
//!
//! The type is `Copy` and never allocates, so it can cross to the audio
//! thread as is.

use std::fmt;
use std::str::FromStr;

use crate::Curve;

/// Slowest and fastest tempo, in beats per minute of the metre's unit.
pub const MIN_BPM: f64 = 20.0;
pub const MAX_BPM: f64 = 300.0;

/// Tempo and metre before anyone sets them.
pub const DEFAULT_BPM: f64 = 120.0;

/// A time signature: `beats` per bar, each a `1/unit` note.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Meter {
    pub beats: u8,
    pub unit: u8,
}

impl Meter {
    pub const COMMON: Meter = Meter { beats: 4, unit: 4 };

    /// Quarter notes in one of this metre's beats (`2` for a half, `0.5` for an eighth).
    pub fn quarters_per_beat(self) -> f64 {
        4.0 / self.unit as f64
    }

    pub fn quarters_per_bar(self) -> f64 {
        self.beats as f64 * self.quarters_per_beat()
    }
}

impl Default for Meter {
    fn default() -> Self {
        Self::COMMON
    }
}

impl fmt::Display for Meter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.beats, self.unit)
    }
}

impl FromStr for Meter {
    type Err = String;

    /// `"4/4"`, `"3/4"`, `"6/8"`, `"7/8"`, `"5/4"`…
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bad = || format!("meter '{s}': write it like \"4/4\", \"3/4\" or \"6/8\"");
        let (b, u) = s.trim().split_once('/').ok_or_else(bad)?;
        let beats: u8 = b.trim().parse().map_err(|_| bad())?;
        let unit: u8 = u.trim().parse().map_err(|_| bad())?;
        if !(1..=32).contains(&beats) {
            return Err(format!("meter '{s}': between 1 and 32 beats per bar"));
        }
        if ![1, 2, 4, 8, 16].contains(&unit) {
            return Err(format!(
                "meter '{s}': the beat unit must be 1, 2, 4, 8 or 16"
            ));
        }
        Ok(Meter { beats, unit })
    }
}

/// The tempo from one change on: `from` quarter-note BPM at `time`, moving
/// to `to` over `ramp` seconds along `curve`, then holding.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TempoMap {
    /// Seconds (director clock) when this segment begins.
    time: f64,
    /// Quarter notes since the clock started, at `time`.
    beat: f64,
    /// In quarter notes per minute.
    from: f64,
    to: f64,
    /// Seconds; 0 means a steady tempo.
    ramp: f64,
    curve: Curve,
    meter: Meter,
    /// The quarter-note position of a downbeat: bars count from here.
    bar_origin: f64,
}

impl Default for TempoMap {
    fn default() -> Self {
        Self {
            time: 0.0,
            beat: 0.0,
            from: DEFAULT_BPM,
            to: DEFAULT_BPM,
            ramp: 0.0,
            curve: Curve::Linear,
            meter: Meter::COMMON,
            bar_origin: 0.0,
        }
    }
}

/// Where the music is: bar 1, beat 1 is the first downbeat.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub bar: i64,
    /// 1-based, in the metre's unit, with the fraction: `2.5` is halfway
    /// between beats 2 and 3.
    pub beat: f64,
}

impl TempoMap {
    pub fn meter(&self) -> Meter {
        self.meter
    }

    /// Quarter notes covered `x` seconds into the segment.
    fn quarters_in(&self, x: f64) -> f64 {
        if x <= 0.0 || self.ramp <= 0.0 {
            return x * self.from / 60.0;
        }
        if x >= self.ramp {
            return self.ramp_quarters() + (x - self.ramp) * self.to / 60.0;
        }
        let area = self.curve.integral(x / self.ramp) * self.ramp;
        (self.from * x + (self.to - self.from) * area) / 60.0
    }

    fn ramp_quarters(&self) -> f64 {
        (self.from * self.ramp + (self.to - self.from) * self.ramp * self.curve.integral(1.0))
            / 60.0
    }

    /// Quarter-note BPM at `t`.
    fn quarter_bpm(&self, t: f64) -> f64 {
        let x = t - self.time;
        if self.ramp <= 0.0 || x <= 0.0 {
            self.from
        } else if x >= self.ramp {
            self.to
        } else {
            self.from + (self.to - self.from) * self.curve.apply((x / self.ramp) as f32) as f64
        }
    }

    /// Quarter notes since the clock started, at `t`.
    pub fn beat_at(&self, t: f64) -> f64 {
        self.beat + self.quarters_in(t - self.time)
    }

    /// When the clock reaches quarter note `b`. Exact outside a ramp; inside
    /// one, solved to well under a sample.
    pub fn time_at(&self, b: f64) -> f64 {
        let q = b - self.beat;
        if q <= 0.0 || self.ramp <= 0.0 {
            return self.time + q * 60.0 / self.from;
        }
        let rq = self.ramp_quarters();
        if q >= rq {
            return self.time + self.ramp + (q - rq) * 60.0 / self.to;
        }
        // Inside the ramp `quarters_in` is increasing (tempo is always
        // positive): safeguarded Newton between 0 and `ramp`.
        let (mut lo, mut hi) = (0.0, self.ramp);
        let mut x = q * 60.0 / ((self.from + self.to) / 2.0);
        for _ in 0..64 {
            if !(lo..=hi).contains(&x) {
                x = (lo + hi) / 2.0;
            }
            let err = self.quarters_in(x) - q;
            if err.abs() < 1e-10 {
                break;
            }
            if err > 0.0 {
                hi = x;
            } else {
                lo = x;
            }
            x -= err * 60.0 / self.quarter_bpm(self.time + x);
        }
        self.time + x
    }

    /// Quarter notes per second at `t`.
    pub fn quarters_per_second(&self, t: f64) -> f64 {
        self.quarter_bpm(t) / 60.0
    }

    /// Tempo at `t`, in the metre's unit per minute.
    pub fn bpm_at(&self, t: f64) -> f64 {
        self.quarter_bpm(t) / self.meter.quarters_per_beat()
    }

    /// The tempo a ramp under way is heading for, and the seconds it has left.
    pub fn ramp_at(&self, t: f64) -> Option<(f64, f64)> {
        let left = self.time + self.ramp - t;
        (self.ramp > 0.0 && left > 0.0).then(|| (self.to / self.meter.quarters_per_beat(), left))
    }

    pub fn position(&self, t: f64) -> Position {
        let q = self.beat_at(t) - self.bar_origin;
        let per_bar = self.meter.quarters_per_bar();
        let bar = (q / per_bar).floor();
        let within = q - bar * per_bar;
        Position {
            bar: bar as i64 + 1,
            beat: within / self.meter.quarters_per_beat() + 1.0,
        }
    }

    /// When bar `n` begins (bar 1 is the first downbeat; numbering restarts
    /// with a new metre or `downbeat`). Later tempo changes can move it.
    pub fn time_of_bar(&self, n: i64) -> f64 {
        self.time_at(self.bar_origin + (n - 1) as f64 * self.meter.quarters_per_bar())
    }

    /// The first downbeat at or after `t`.
    pub fn next_bar(&self, t: f64) -> f64 {
        self.next_multiple(t, self.meter.quarters_per_bar())
    }

    /// The first beat at or after `t`.
    pub fn next_beat(&self, t: f64) -> f64 {
        self.next_multiple(t, self.meter.quarters_per_beat())
    }

    fn next_multiple(&self, t: f64, step: f64) -> f64 {
        let q = self.beat_at(t) - self.bar_origin;
        // Within a microsecond of a grid line counts as on it.
        let n = ((q - 1e-6 * self.quarter_bpm(t) / 60.0) / step).ceil();
        self.time_at(self.bar_origin + n * step)
    }

    /// Seconds a bar lasts at `t`'s tempo.
    pub fn seconds_per_bar(&self, t: f64) -> f64 {
        self.meter.quarters_per_bar() * 60.0 / self.quarter_bpm(t)
    }

    /// The map after a change at `t`. The beat position carries on
    /// unbroken, so nothing already counted moves.
    ///
    /// - `bpm`: the new tempo in the new metre's unit; `None` keeps the
    ///   current speed of the quarter note.
    /// - `meter`: a new metre starts a new bar at `t`.
    /// - `ramp`: glide to `bpm` over these seconds along `curve` instead of
    ///   switching at once (0 switches).
    /// - `downbeat`: make `t` the downbeat of bar 1.
    pub fn changed(
        &self,
        t: f64,
        bpm: Option<f64>,
        meter: Option<Meter>,
        ramp: f64,
        curve: Curve,
        downbeat: bool,
    ) -> TempoMap {
        let beat = self.beat_at(t);
        let now_q = self.quarter_bpm(t);
        let new_meter = meter.unwrap_or(self.meter);
        let to = bpm.map_or(now_q, |b| b * new_meter.quarters_per_beat());
        let ramp = if to == now_q { 0.0 } else { ramp.max(0.0) };
        TempoMap {
            time: t,
            beat,
            // A change without a ramp switches at once.
            from: if ramp > 0.0 { now_q } else { to },
            to,
            ramp,
            curve,
            meter: new_meter,
            bar_origin: if downbeat || meter.is_some_and(|m| m != self.meter) {
                beat
            } else {
                self.bar_origin
            },
        }
    }

    /// Seconds a ramp from the tempo at `t` to `bpm` (metre unit) must take
    /// to cover `quarters`, along `curve`.
    pub fn ramp_seconds_for(
        &self,
        t: f64,
        bpm: f64,
        meter: Option<Meter>,
        quarters: f64,
        curve: Curve,
    ) -> f64 {
        let from = self.quarter_bpm(t);
        let to = bpm * meter.unwrap_or(self.meter).quarters_per_beat();
        let mean = from + (to - from) * curve.integral(1.0);
        quarters * 60.0 / mean
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tol: f64) -> bool {
        (a - b).abs() < tol
    }

    #[test]
    fn meters_parse() {
        assert_eq!("4/4".parse(), Ok(Meter { beats: 4, unit: 4 }));
        assert_eq!(" 6 / 8 ".parse(), Ok(Meter { beats: 6, unit: 8 }));
        assert!("4/3".parse::<Meter>().is_err());
        assert!("four".parse::<Meter>().is_err());
        assert_eq!(Meter { beats: 6, unit: 8 }.quarters_per_bar(), 3.0);
        assert_eq!(Meter { beats: 7, unit: 8 }.to_string(), "7/8");
    }

    #[test]
    fn steady_tempo_counts_bars() {
        let m = TempoMap::default().changed(0.0, Some(138.0), None, 0.0, Curve::Linear, true);
        let bar = 4.0 * 60.0 / 138.0;
        assert!(close(m.seconds_per_bar(5.0), bar, 1e-12));
        assert!(close(m.time_at(8.0), 2.0 * bar, 1e-12));
        let p = m.position(2.0 * bar + bar / 2.0);
        assert_eq!(p.bar, 3);
        assert!(close(p.beat, 3.0, 1e-9));
    }

    #[test]
    fn six_eight_counts_eighths() {
        let m = TempoMap::default().changed(
            0.0,
            Some(180.0),
            Some("6/8".parse().unwrap()),
            0.0,
            Curve::Linear,
            false,
        );
        // 180 eighths a minute: a 6/8 bar lasts 2 s.
        assert!(close(m.seconds_per_bar(0.0), 2.0, 1e-12));
        assert!(close(m.bpm_at(1.0), 180.0, 1e-9));
        let p = m.position(3.0 + 1.0 / 3.0);
        assert_eq!(p.bar, 2);
        assert!(close(p.beat, 5.0, 1e-9));
    }

    #[test]
    fn ramps_cover_the_right_number_of_beats() {
        for curve in Curve::ALL {
            let base = TempoMap::default();
            // 4 bars of 4/4 from 120 to 60.
            let secs = base.ramp_seconds_for(10.0, 60.0, None, 16.0, curve);
            let m = base.changed(10.0, Some(60.0), None, secs, curve, false);
            let start = m.beat_at(10.0);
            assert!(
                close(m.beat_at(10.0 + secs) - start, 16.0, 1e-9),
                "{curve:?}"
            );
            assert!(
                close(m.time_at(start + 16.0), 10.0 + secs, 1e-9),
                "{curve:?}"
            );
            // A ritardando takes longer than four bars at the old tempo.
            assert!(secs > 8.0, "{curve:?}");
            // Inside the ramp time_at inverts beat_at to well under a sample.
            for i in 1..40 {
                let t = 10.0 + secs * i as f64 / 40.0;
                assert!(close(m.time_at(m.beat_at(t)), t, 1e-7), "{curve:?} at {t}");
            }
            assert!(close(m.bpm_at(10.0 + secs + 1.0), 60.0, 1e-9));
        }
    }

    #[test]
    fn changes_never_move_the_beat() {
        let a = TempoMap::default();
        let b = a.changed(7.3, Some(90.0), None, 5.0, Curve::EaseInOut, false);
        assert!(close(a.beat_at(7.3), b.beat_at(7.3), 1e-12));
        // A change in the middle of a ramp starts from the tempo of that moment.
        let mid = b.bpm_at(9.0);
        let c = b.changed(9.0, Some(150.0), None, 4.0, Curve::Linear, false);
        assert!(close(c.bpm_at(9.0), mid, 1e-9));
        assert!(close(c.beat_at(9.0), b.beat_at(9.0), 1e-12));
    }

    #[test]
    fn bars_by_number() {
        let m = TempoMap::default().changed(5.0, Some(60.0), None, 0.0, Curve::Linear, true);
        // 60 BPM 4/4 from 5 s: bar 3 starts 8 s later.
        assert!(close(m.time_of_bar(1), 5.0, 1e-12));
        assert!(close(m.time_of_bar(3), 13.0, 1e-12));
        assert_eq!(m.position(m.time_of_bar(7)).bar, 7);
    }

    #[test]
    fn grid_lines() {
        let m = TempoMap::default(); // 120 BPM 4/4: a bar every 2 s
        assert!(close(m.next_bar(0.0), 0.0, 1e-12));
        assert!(close(m.next_bar(0.1), 2.0, 1e-12));
        assert!(close(m.next_beat(2.1), 2.5, 1e-12));
        let m = m.changed(3.0, None, None, 0.0, Curve::Linear, true);
        assert!(close(m.next_bar(3.0), 3.0, 1e-12));
        assert_eq!(m.position(3.0).bar, 1);
        assert!(close(m.next_bar(3.2), 5.0, 1e-12));
    }
}
