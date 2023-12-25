//! Parsing the XML into something usable in Rust.
//!
//! Note that anytime `Id` is mentioned, actually the `guid` attribute is meant, **not** the `id` one.

use std::collections::{BTreeMap, HashMap};

use eyre::Result;
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
struct Person {
    pub id: PersonId,
    pub name: String,
}

impl Schedule {
    pub fn new() -> Result<Self> {
        Ok(Self::placeholder())
    }

    pub fn placeholder() -> Self {
        let person_id = PersonId(Uuid::max());
        let person = Person {
            id: person_id,
            name: "Semi-non-professional yak shaver".to_string(),
        };

        let event_id = EventId(Uuid::nil());
        let event_start = DateTime::now_utc();
        let event = Event {
            id: event_id,
            start: event_start,
            duration: Duration::new(5 * 60, 0),
            title: "Shaving yaks".to_string(),
            subtitle: "Important. I think.".to_string(),
            r#abstract: "We shave yaks, which is somewhat common in software development.".to_string(),
            description: "Recent advances in yak shaving have shown an overall global increase in yak shaving, so we will look at how that happened.".to_string(),
            room: "Somewhere I belong".to_string(),
            track: "Science".to_string(),
            r#type: "lecture".to_string(),
            language: "en".to_string(),
            url: "https://example.com".to_string(),
            feedback_url: "https://example.com".to_string(),
            links: BTreeMap::new(),
            persons: Vec::new(),
        };

        Self {
            events: [(event_id, event)].into_iter().collect(),
            persons: [(person_id, person)].into_iter().collect(),
            time_map: [(event_start, vec![event_id])].into_iter().collect(),
        }
    }

    /// Returns the first event in this schedule, or `None` if the schedule contains no events.
    pub fn first(&self) -> Option<&Event> {
        let id = self
            .time_map
            .first_key_value()?
            .1
            .first()
            .expect("time map must be consistent with events");

        Some(self.lookup(id))
    }

    fn lookup(&self, id: &EventId) -> &Event {
        // theoretically it's possible to accidentally use an EventId for another schedule in here
        // though we never construct another schedule in-app and this function is only called for EventIds contained in this schedule
        self.events
            .get(id)
            .expect("schedule to be immutable once constructed")
    }
}

impl Event {
    fn end(&self) -> DateTime {
        self.start + self.duration
    }
}
