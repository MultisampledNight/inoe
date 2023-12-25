use eyre::{Context, Result};

use super::{Action, Schedule};

pub struct Store {
    state: State,
}

pub struct State {
    pub schedule: Schedule,
}

impl Store {
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: State::new()?,
        })
    }

    pub fn update(&mut self, _action: Action) {}

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl State {
    pub fn new() -> Result<Self> {
        Ok(Self {
            schedule: Schedule::new().context("schedule construction failure")?,
        })
    }
}
