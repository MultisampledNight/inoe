//! Parse the XML by first throwing it into the appropiate serde mapping, then walking through
//! that mapping and extracting what we actually need.

use serde::Deserialize;
use time::{OffsetDateTime, Time};
use uuid::Uuid;

use crate::DateTime;

time::serde::format_description!(time_only, Time, "[hour]:[minute]");
time::serde::format_description!(
    version_timestamp,
    OffsetDateTime,
    "[year]-[month]-[day] [hour]:[minute]"
);

#[derive(Debug, Deserialize)]
pub struct Schedule {
    // #[serde(with = "version_timestamp")]
    // pub version: OffsetDateTime,
    pub conference: Conference,
    #[serde(rename = "day")]
    pub days: Vec<Day>,
}

#[derive(Debug, Deserialize)]
pub struct Conference {
    pub acronym: String,
    pub title: String,
    #[serde(with = "time::serde::rfc3339")]
    pub start: DateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub end: DateTime,
    pub url: String,

    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@color")]
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct Day {
    #[serde(rename = "room")]
    pub rooms: Vec<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    #[serde(rename = "@guid")]
    pub guid: Uuid,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "event")]
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "@guid")]
    pub guid: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date: DateTime,
    #[serde(with = "time_only")]
    pub duration: Time,

    pub room: String,
    pub title: String,
    pub subtitle: String,
    pub language: String,
    pub track: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "abstract")]
    pub r#abstract: String,
    pub description: String,
    pub persons: Persons,
}

#[derive(Debug, Deserialize)]
pub struct Persons {
    #[serde(default)]
    pub person: Vec<Person>,
}

#[derive(Debug, Deserialize)]
pub struct Person {
    #[serde(rename = "@guid")]
    pub guid: Uuid,
    #[serde(rename = "$value")]
    pub name: String,
}
