//! Chained sections: a cue list the conductor runs on its own.
//!
//! A [`CueList`] is a list of sections, each so many bars (or seconds) long,
//! each with actions that happen at its start or some bars into it. The
//! actions themselves are opaque here: closures the layer above has already
//! validated, called with the moment they are for. Each says how far ahead
//! it must be set up (`lead`): a harmony, a tempo change, a sequence or a
//! phrase is set up a little before its bar so that everything worked out
//! ahead already knows it, and lands exactly there; a morph or a gesture is
//! fired on the moment.
//!
//! Sections are measured as they come: when one starts, the start of the
//! next is worked out from the tempo map then, so a tempo change inside a
//! section stretches it in seconds but not in bars.

use std::collections::VecDeque;
use std::sync::Arc;

use serde::Serialize;

use crate::{Conductor, StartOn, TempoMap};

/// What an action does when its moment comes: called with the conductor,
/// the time now and where it lands (a beat or a time, never passed).
pub type CueRun = Arc<dyn Fn(&Conductor, f64, StartOn) -> Result<(), String> + Send + Sync>;

/// How far into its section an action happens, or how long a section is.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Span {
    Bars(f64),
    Seconds(f64),
}

/// One thing a section does.
#[derive(Clone)]
pub struct CueAction {
    /// What it does, in words, for the reports.
    pub what: String,
    /// Seconds before its moment it is set up.
    pub lead: f64,
    /// Among actions set up together, lower goes first (tempo before the
    /// harmony before the notes).
    pub order: u8,
    /// How far into the section.
    pub after: Span,
    pub run: CueRun,
}

/// A section: a name, a length and what happens in it.
#[derive(Clone)]
pub struct CueSection {
    pub name: String,
    pub length: Span,
    pub actions: Vec<CueAction>,
}

/// The sections, in order.
#[derive(Clone)]
pub struct CueList {
    pub sections: Vec<CueSection>,
    /// Start again from the first when the last ends.
    pub looping: bool,
}

/// A moment: a quarter-note position (beats never move, whatever the tempo
/// does) or a time on the director's clock.
#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Point {
    Quarter(f64),
    Time(f64),
}

impl Point {
    pub(crate) fn time(self, tempo: &TempoMap) -> f64 {
        match self {
            Point::Quarter(q) => tempo.time_at(q),
            Point::Time(t) => t,
        }
    }

    fn quarter(self, tempo: &TempoMap) -> f64 {
        match self {
            Point::Quarter(q) => q,
            Point::Time(t) => tempo.beat_at(t),
        }
    }

    /// This moment plus a span, bars counted in the metre in force here.
    fn plus(self, span: Span, tempo: &TempoMap) -> Point {
        match span {
            Span::Bars(b) => {
                let t = self.time(tempo);
                let per_bar = tempo.meter_at(t + 1e-6).quarters_per_bar();
                Point::Quarter(self.quarter(tempo) + b * per_bar)
            }
            Span::Seconds(s) => Point::Time(self.time(tempo) + s),
        }
    }

    fn start_on(self) -> StartOn {
        match self {
            Point::Quarter(q) => StartOn::AtQuarter(q),
            Point::Time(t) => StartOn::AtTime(t),
        }
    }
}

/// An action waiting for its moment, or a section waiting to begin.
#[derive(Clone, Debug)]
struct Waiting {
    at: Point,
    lead: f64,
    order: u8,
    section: usize,
    /// None: the section's own start.
    action: Option<usize>,
}

/// The cue list as it runs.
pub(crate) struct Running {
    list: CueList,
    waiting: Vec<Waiting>,
    /// The section playing and when it began.
    current: Option<(usize, Point)>,
    /// The next section and when it begins.
    upcoming: Option<(usize, Point)>,
    /// When the last section ends, if the list does not loop.
    ends: Option<Point>,
    /// When each section of this lap began or is set to begin (the rest
    /// follow from the one before).
    starts: Vec<Option<Point>>,
    /// Sections begun since the conductor last asked, for its log.
    begun: Vec<(usize, Point)>,
    errors: VecDeque<String>,
}

/// Most errors kept for the reports.
const ERRORS_KEPT: usize = 8;

/// What the cue list is doing, for the reports.
#[derive(Clone, Debug, Serialize)]
pub struct CueState {
    /// The section playing (none before the first begins).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Its bar now, counting from 1 (for a section in bars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<f64>,
    /// How long it is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub of_bars: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub of_seconds: Option<f64>,
    /// The section coming next, and the seconds until it begins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_in: Option<f64>,
    /// Seconds until the last section ends (if it does not loop).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_in: Option<f64>,
    #[serde(rename = "loop")]
    pub looping: bool,
    /// Every section, in order, with the bar it begins on.
    pub sections: Vec<SectionInfo>,
    /// Actions that failed when their moment came (section: action: why).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
}

/// A section in the reports.
#[derive(Clone, Debug, Serialize)]
pub struct SectionInfo {
    pub name: String,
    /// The bar it begins on in this lap, by the count now (a tempo or metre
    /// change still to come can move the later ones).
    pub bar: i64,
}

fn secs(x: f64) -> f64 {
    (x.max(0.0) * 100.0).round() / 100.0
}

impl Running {
    /// Start `list` with its first section at `start`.
    pub(crate) fn new(list: CueList, start: Point, tempo: &TempoMap) -> Self {
        let n = list.sections.len();
        let mut r = Running {
            list,
            waiting: Vec::new(),
            current: None,
            upcoming: None,
            ends: None,
            starts: vec![None; n],
            begun: Vec::new(),
            errors: VecDeque::new(),
        };
        r.queue_section(0, start, tempo);
        r
    }

    fn queue_section(&mut self, k: usize, start: Point, tempo: &TempoMap) {
        self.upcoming = Some((k, start));
        // The first again, once a section has played, is the next lap.
        if k > 0 || self.current.is_none() {
            self.starts[k] = Some(start);
        }
        self.waiting.push(Waiting {
            at: start,
            lead: 0.0,
            order: 0,
            section: k,
            action: None,
        });
        for (i, a) in self.list.sections[k].actions.iter().enumerate() {
            let at = match a.after {
                Span::Bars(0.0) | Span::Seconds(0.0) => start,
                span => start.plus(span, tempo),
            };
            self.waiting.push(Waiting {
                at,
                lead: a.lead,
                order: a.order,
                section: k,
                action: Some(i),
            });
        }
    }

    /// Everything due by `now`: sections that begin (the next one is queued
    /// as each begins), and the actions to run, in order, with where each
    /// lands and what it is called in the reports.
    pub(crate) fn due(&mut self, now: f64, tempo: &TempoMap) -> Vec<(CueRun, StartOn, String)> {
        let mut out = Vec::new();
        // A section beginning queues the next, whose actions may be due at
        // once (a short section): go round until nothing more is due.
        for _ in 0..64 {
            let mut due: Vec<(f64, Waiting)> = Vec::new();
            self.waiting.retain(|w| {
                let fire = w.at.time(tempo) - w.lead;
                if now >= fire {
                    due.push((fire, w.clone()));
                    false
                } else {
                    true
                }
            });
            if due.is_empty() {
                break;
            }
            due.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.order.cmp(&b.1.order)));
            for (_, w) in due {
                match w.action {
                    None => self.begin(w.section, w.at, tempo),
                    Some(i) => {
                        let s = &self.list.sections[w.section];
                        let a = &s.actions[i];
                        out.push((
                            a.run.clone(),
                            w.at.start_on(),
                            format!("{}: {}", s.name, a.what),
                        ));
                    }
                }
            }
        }
        out
    }

    fn begin(&mut self, k: usize, at: Point, tempo: &TempoMap) {
        if k == 0 {
            self.starts.iter_mut().for_each(|s| *s = None);
        }
        self.starts[k] = Some(at);
        self.begun.push((k, at));
        self.current = Some((k, at));
        let end = at.plus(self.list.sections[k].length, tempo);
        let n = self.list.sections.len();
        if k + 1 < n {
            self.queue_section(k + 1, end, tempo);
        } else if self.list.looping {
            self.queue_section(0, end, tempo);
        } else {
            self.upcoming = None;
            self.ends = Some(end);
        }
    }

    pub(crate) fn failed(&mut self, error: String) {
        if self.errors.len() == ERRORS_KEPT {
            self.errors.pop_front();
        }
        self.errors.push_back(error);
    }

    /// The sections begun since the last call: their names and when.
    pub(crate) fn take_begun(&mut self, tempo: &TempoMap) -> Vec<(String, f64)> {
        self.begun
            .drain(..)
            .map(|(k, at)| (self.list.sections[k].name.clone(), at.time(tempo)))
            .collect()
    }

    /// When the last section ends, if the list does not loop and it has
    /// begun.
    pub(crate) fn end_time(&self, tempo: &TempoMap) -> Option<f64> {
        self.ends.map(|e| e.time(tempo))
    }

    /// When the section called `name` plays: the one playing now if it is
    /// that one, else the next to come (within a lap). Start and end.
    pub(crate) fn next_occurrence(&self, name: &str, tempo: &TempoMap) -> Option<(f64, f64)> {
        let n = self.list.sections.len();
        let (mut k, mut at) = self.current.or(self.upcoming)?;
        for _ in 0..=n {
            let s = &self.list.sections[k];
            let end = at.plus(s.length, tempo);
            if s.name == name {
                return Some((at.time(tempo), end.time(tempo)));
            }
            k += 1;
            if k == n {
                if !self.list.looping {
                    return None;
                }
                k = 0;
            }
            at = end;
        }
        None
    }

    /// The names of the sections, in order.
    pub(crate) fn names(&self) -> Vec<String> {
        self.list.sections.iter().map(|s| s.name.clone()).collect()
    }

    /// True once the last section (of a list that does not loop) has ended.
    pub(crate) fn finished(&self, now: f64, tempo: &TempoMap) -> bool {
        self.waiting.is_empty() && self.ends.is_some_and(|e| now >= e.time(tempo))
    }

    pub(crate) fn state(&self, now: f64, tempo: &TempoMap) -> CueState {
        let mut sections = Vec::with_capacity(self.list.sections.len());
        let mut next = None;
        for (s, known) in self.list.sections.iter().zip(&self.starts) {
            let at = known.or(next).unwrap_or(Point::Time(now));
            sections.push(SectionInfo {
                name: s.name.clone(),
                bar: tempo.position(at.time(tempo) + 1e-6).bar,
            });
            next = Some(at.plus(s.length, tempo));
        }
        let current = self.current.map(|(k, at)| (&self.list.sections[k], at));
        let (bar, of_bars, of_seconds) = match current {
            Some((s, at)) => match s.length {
                Span::Bars(b) => {
                    let per_bar = tempo.meter_at(now).quarters_per_bar();
                    let into = (tempo.beat_at(now) - at.quarter(tempo)) / per_bar;
                    (Some((into.floor() + 1.0).min(b.ceil())), Some(b), None)
                }
                Span::Seconds(s) => (None, None, Some(s)),
            },
            None => (None, None, None),
        };
        CueState {
            section: current.map(|(s, _)| s.name.clone()),
            bar,
            of_bars,
            of_seconds,
            next: self
                .upcoming
                .map(|(k, _)| self.list.sections[k].name.clone()),
            next_in: self.upcoming.map(|(_, at)| secs(at.time(tempo) - now)),
            ends_in: self.ends.map(|e| secs(e.time(tempo) - now)),
            looping: self.list.looping,
            sections,
            errors: self.errors.iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Curve;
    use std::sync::Mutex;

    /// An action that records the moment it was run with.
    fn rec(log: &Arc<Mutex<Vec<String>>>, what: &str, lead: f64, after: Span) -> CueAction {
        let log = log.clone();
        let name = what.to_string();
        CueAction {
            what: what.into(),
            lead,
            order: 0,
            after,
            run: Arc::new(move |_c, _now, on| {
                log.lock().unwrap().push(format!("{name}@{on:?}"));
                Ok(())
            }),
        }
    }

    #[test]
    fn sections_follow_each_other_and_actions_land_on_their_beats() {
        let tempo = TempoMap::default().changed(0.0, Some(120.0), None, 0.0, Curve::Linear, true);
        let log = Arc::new(Mutex::new(Vec::new()));
        let list = CueList {
            sections: vec![
                CueSection {
                    name: "a".into(),
                    length: Span::Bars(2.0),
                    actions: vec![
                        rec(&log, "harmony", 0.6, Span::Bars(0.0)),
                        rec(&log, "gesture", 0.0, Span::Bars(1.0)),
                    ],
                },
                CueSection {
                    name: "b".into(),
                    length: Span::Seconds(3.0),
                    actions: vec![rec(&log, "seq", 0.6, Span::Bars(0.0))],
                },
            ],
            looping: false,
        };
        // Bar 2 begins at 2 s: the list starts there.
        let mut r = Running::new(list, Point::Quarter(4.0), &tempo);
        let mut fired = Vec::new();
        for i in 0..1000 {
            let now = i as f64 * 0.01;
            for (_run, on, what) in r.due(now, &tempo) {
                fired.push((now, what, on));
            }
        }
        let at = |w: &str| fired.iter().find(|f| f.1 == w).unwrap();
        // The harmony is set up 0.6 s before section a, for its beat.
        let h = at("a: harmony");
        assert!((h.0 - 1.4).abs() < 0.011 && h.2 == StartOn::AtQuarter(4.0));
        // The gesture fires on its bar (a bar in: 4 s).
        let g = at("a: gesture");
        assert!((g.0 - 4.0).abs() < 0.011 && g.2 == StartOn::AtQuarter(8.0));
        // Section b begins two bars after a (6 s); its sequence 0.6 s ahead.
        let s = at("b: seq");
        assert!((s.0 - 5.4).abs() < 0.011 && s.2 == StartOn::AtQuarter(12.0));
        assert_eq!(fired.len(), 3);
        // It ends three seconds after b begins.
        assert!(!r.finished(8.9, &tempo));
        assert!(r.finished(9.0, &tempo));
        let st = r.state(7.0, &tempo);
        assert_eq!(st.section.as_deref(), Some("b"));
        assert!(st.next.is_none() && st.ends_in == Some(2.0));
    }

    #[test]
    fn a_loop_starts_over_and_reports_where_it_is() {
        let tempo = TempoMap::default(); // 120 BPM, a bar every 2 s
        let log = Arc::new(Mutex::new(Vec::new()));
        let list = CueList {
            sections: vec![
                CueSection {
                    name: "verse".into(),
                    length: Span::Bars(2.0),
                    actions: vec![rec(&log, "v", 0.0, Span::Bars(0.0))],
                },
                CueSection {
                    name: "chorus".into(),
                    length: Span::Bars(1.0),
                    actions: vec![rec(&log, "c", 0.0, Span::Bars(0.0))],
                },
            ],
            looping: true,
        };
        let mut r = Running::new(list, Point::Quarter(0.0), &tempo);
        let mut whats = Vec::new();
        for i in 0..=1300 {
            let now = i as f64 * 0.01;
            whats.extend(r.due(now, &tempo).into_iter().map(|d| d.2));
        }
        // 6 s a lap: verse at 0, 6, 12; chorus at 4, 10.
        assert_eq!(
            whats,
            ["verse: v", "chorus: c", "verse: v", "chorus: c", "verse: v"]
        );
        let st = r.state(2.5, &tempo);
        assert_eq!(st.section.as_deref(), Some("verse"));
        assert_eq!(st.next.as_deref(), Some("chorus"));
        let st = r.state(13.0, &tempo);
        assert_eq!((st.bar, st.of_bars), (Some(1.0), Some(2.0)));
        assert!(!r.finished(100.0, &tempo));
    }

    #[test]
    fn the_reports_say_where_each_section_of_this_lap_begins() {
        let tempo = TempoMap::default(); // 120 BPM, a bar every 2 s
        let section = |name: &str, bars| CueSection {
            name: name.into(),
            length: Span::Bars(bars),
            actions: vec![],
        };
        let list = CueList {
            sections: vec![section("verse", 2.0), section("chorus", 1.0)],
            looping: true,
        };
        // Bar 2 is the first downbeat to come.
        let mut r = Running::new(list, Point::Quarter(4.0), &tempo);
        let bars = |r: &Running, now| -> Vec<(String, i64)> {
            r.state(now, &tempo)
                .sections
                .into_iter()
                .map(|s| (s.name, s.bar))
                .collect()
        };
        let lap = |v: i64, c: i64| vec![("verse".to_string(), v), ("chorus".to_string(), c)];
        assert_eq!(bars(&r, 0.5), lap(2, 4));
        for i in 0..=1050 {
            r.due(i as f64 * 0.01, &tempo);
        }
        // The second lap: the verse from 8 s (bar 5), the chorus at 12 s;
        // the next verse is queued for 14 s but this lap still reads 5, 7.
        assert_eq!(bars(&r, 10.5), lap(5, 7));
        for i in 1051..=1450 {
            r.due(i as f64 * 0.01, &tempo);
        }
        assert_eq!(bars(&r, 14.5), lap(8, 10));
    }

    #[test]
    fn a_tempo_change_stretches_a_section_in_seconds_not_bars() {
        // 120 BPM, then from quarter 4 (2 s), 60: bars of 4 s.
        let tempo = TempoMap::default()
            .scheduled(
                0.0,
                4.0,
                Some(60.0),
                None,
                crate::tempo::Ramp::Seconds(0.0),
                Curve::Linear,
                false,
            )
            .unwrap();
        let log = Arc::new(Mutex::new(Vec::new()));
        let list = CueList {
            sections: vec![
                CueSection {
                    name: "slow".into(),
                    length: Span::Bars(1.0),
                    actions: vec![],
                },
                CueSection {
                    name: "after".into(),
                    length: Span::Bars(1.0),
                    actions: vec![rec(&log, "x", 0.0, Span::Bars(0.0))],
                },
            ],
            looping: false,
        };
        let mut r = Running::new(list, Point::Quarter(4.0), &tempo);
        let mut when = None;
        for i in 0..1000 {
            let now = i as f64 * 0.01;
            if !r.due(now, &tempo).is_empty() {
                when = Some(now);
            }
        }
        // "slow" lasts one 4-second bar: "after" begins at 6 s.
        assert!((when.unwrap() - 6.0).abs() < 0.011, "{when:?}");
    }
}
