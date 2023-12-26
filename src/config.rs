use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// XML file of the schedule.
    /// Download it from <https://fahrplan.events.ccc.de/congress/2023/fahrplan/schedule.xml> if not done yet.
    pub schedule: PathBuf,
}

pub fn parse() -> Config {
    Config::parse()
}
