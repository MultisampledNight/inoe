//! Parsing the XML into something usable in Rust.
//!
//! Note that anytime `Id` is mentioned, actually the `guid` attribute is meant, **not** the `id` one.

mod model;

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::BufReader,
    path::PathBuf,
};

use eyre::{Context, Result};
use time::Duration;
use uuid::Uuid;

use crate::DateTime;

#[derive(Clone, Debug)]
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
    pub feedback_url: String,
    pub links: BTreeMap<String, String>,

    pub persons: Vec<PersonId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersonId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
}

impl Schedule {
    pub fn from_xml_file(source: PathBuf) -> Result<Self> {
        let source = fs::File::open(source).context("could not open requested schedule")?;
        let source = BufReader::new(source);

        let raw: model::Schedule =
            quick_xml::de::from_reader(source).context("could not parse schedule")?;

        dbg!(raw);
        todo!()
    }

    /// Returns the first event in this schedule, or `None` if the schedule contains no events.
    pub fn first(&self) -> Option<&Event> {
        let id = self
            .time_map
            .first_key_value()?
            .1
            .first()
            .expect("time map must be consistent with events");

        Some(self.resolve_event(id))
    }

    /// Return the corresponding [`Event`] for the given [`EventId`].
    ///
    /// # Panics
    ///
    /// Panics if the given [`EventId`] is not from this schedule.
    pub fn resolve_event(&self, id: &EventId) -> &Event {
        self.events
            .get(id)
            .expect("EventId to be from the current schedule")
    }

    /// Return the corresponding [`Person`] for the given [`PersonId`].
    ///
    /// # Panics
    ///
    /// Panics if the given [`PersonId`] is not from this schedule.
    pub fn resolve_person(&self, id: &PersonId) -> &Person {
        self.persons
            .get(id)
            .expect("PersonId to be from the current schedule")
    }
}

impl Event {
    pub fn end(&self) -> DateTime {
        self.start + self.duration
    }
}
