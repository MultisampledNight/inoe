use time::{Duration, Time};

use super::model;

impl From<model::Schedule> for super::Schedule {
    /// Walks through the entire schedule and formats it in such a way that it becomes usable.
    fn from(model: model::Schedule) -> Self {
        let mut schedule = Self::default();

        let events = model
            .days
            .into_iter()
            .flat_map(|day| day.rooms)
            .flat_map(|room| room.events);
        for event in events {
            let (event, persons) = realize_event(event);

            schedule
                .time_map
                .entry(event.start)
                .or_default()
                .push(event.id);
            schedule.events.insert(event.id, event);

            schedule
                .persons
                .extend(persons.into_iter().map(|person| (person.id, person)));
        }

        schedule
    }
}

fn realize_event(model: model::Event) -> (super::Event, Vec<super::Person>) {
    let (person_ids, persons): (Vec<_>, Vec<_>) = model
        .persons
        .persons
        .into_iter()
        .map(realize_person)
        .map(|person| (person.id, person))
        .unzip();

    let event = super::Event {
        id: super::EventId(model.guid),
        start: model.date,
        duration: time_to_duration(model.duration),
        title: model.title,
        subtitle: model.subtitle,
        r#abstract: model.r#abstract,
        description: model.description,
        room: model.room,
        track: model.track,
        r#type: model.r#type,
        language: model.language,
        url: model.url,
        feedback_url: model.feedback_url,
        links: model
            .links
            .links
            .into_iter()
            .map(|link| (link.display, link.href))
            .collect(),
        persons: person_ids,
    };

    (event, persons)
}

fn time_to_duration(time: Time) -> Duration {
    let (hours, minutes, seconds) = time.as_hms();
    Duration::seconds(i64::from(hours) * 60 * 60 + i64::from(minutes) * 60 + i64::from(seconds))
}

fn realize_person(model: model::Person) -> super::Person {
    super::Person {
        id: super::PersonId(model.guid),
        name: model.name,
    }
}
