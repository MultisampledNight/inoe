//! Parsing the XML into something usable in Rust.
//!
//! Note that anytime `Id` is mentioned, actually the `guid` attribute is meant, **not** the `id` one.
//! The pipeline is `XML` → [`model::Schedule`] → [`convert`]'s [`From`] impl → [`Schedule`].

pub mod convert;
pub mod model;

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::BufReader,
    ops::Index,
    path::Path,
};

use either::Either;
use eyre::{Context, Result};
use time::Duration;
use uuid::Uuid;

use crate::DateTime;

#[derive(Clone, Debug, Default)]
pub struct Schedule {
    events: HashMap<EventId, Event>,
    persons: HashMap<PersonId, Person>,

    time_map: BTreeMap<DateTime, Vec<EventId>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    pub id: EventId,

    pub start: DateTime,
    pub duration: Duration,

    pub title: String,
    pub subtitle: String,
    pub r#abstract: String,
    pub description: String,

    pub room: String,
    pub track: String,
    pub r#type: String,

    pub language: String,
    pub url: String,
    pub feedback_url: Option<String>,
    /// Displayed text is key, URL is value.
    pub links: BTreeMap<String, String>,

    pub persons: Vec<PersonId>,
}

impl Event {
    pub fn end(&self) -> DateTime {
        self.start + self.duration
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
}

impl Schedule {
    pub fn from_xml_file(source: impl AsRef<Path>) -> Result<Self> {
        let source = fs::File::open(source).context("could not open requested schedule")?;
        let source = BufReader::new(source);

        let model = model::parse(source).context("could not parse schedule into model")?;
        let schedule = model.into();

        Ok(schedule)
    }

    /// Pure getter.
    pub fn time_map(&self) -> &BTreeMap<DateTime, Vec<EventId>> {
        &self.time_map
    }

    /// Returns the first event in this schedule, or `None` if the schedule contains no events.
    pub fn first(&self) -> Option<&Event> {
        let id = self
            .time_map
            .first_key_value()?
            .1
            .first()
            .expect("time map must be consistent with events");

        Some(&self[id])
    }

    /// Returns the requested _n_-th date and events after to the given date.
    /// Negative _n_ result in the date and events _before_ the given date.
    ///
    /// Returns [`None`] if one of
    ///
    /// 1. there is no such event (exceeded range of available events)
    /// 2. `n == 0` and `to` doesn't point to an existing timeslot start
    pub fn relative(&mut self, n: isize, to: DateTime) -> Option<(&DateTime, &Vec<EventId>)> {
        // see which direction we need to look to
        let mut iter = match n.signum() {
            -1 => {
                // negative, so _before_
                Either::Left(self.time_map.range(..to).rev())
            }
            1 => {
                // positive, so _after_
                Either::Right(self.time_map.range(to..))
            }
            0 => return self.time_map.get_key_value(&to),
            _ => unreachable!("signum never returns outside of [-1, 1]"),
        };

        let n = n.abs() as usize;

        iter.nth(n)
    }
}

impl Index<&EventId> for Schedule {
    type Output = Event;

    /// May panic or return the wrong event if the given [`EventId`] does not belong to this
    /// schedule.
    fn index(&self, id: &EventId) -> &Self::Output {
        self.events
            .get(id)
            .expect("EventId to be from the current schedule")
    }
}

impl Index<&PersonId> for Schedule {
    type Output = Person;

    /// May panic or return the wrong event if the given [`PersonId`] does not belong to this
    /// schedule.
    fn index(&self, id: &PersonId) -> &Self::Output {
        self.persons
            .get(id)
            .expect("PersonId to be from the current schedule")
    }
}

impl Index<&TimeCoord> for Schedule {
    type Output = Event;
    fn index(&self, index: &TimeCoord) -> &Self::Output {
        let id = self.time_map[&index.row][index.idx];
        &self[&id]
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TimeCoord {
    pub row: DateTime,
    pub idx: usize,
}
