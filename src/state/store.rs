use eyre::{Context, ContextCompat, Result};

use crate::DateTime;

use super::{schedule::EventId, schedule::Schedule, Action};

pub struct Store {
    state: State,
}

pub struct State {
    pub schedule: Schedule,
    pub view: View,
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
        let schedule = Schedule::new().context("schedule construction failure")?;
        let first_event = schedule
            .first()
            .context("schedule is empty, nothing to display")?;
        let view = View::GridOverview {
            scroll_at: first_event.start,
            selected: first_event.id,
        };

        Ok(Self { schedule, view })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum View {
    GridOverview {
        /// Topmost point of where the scroll currently is.
        scroll_at: DateTime,
        /// What event is currently selected and would be viewed if switched into [`View::Single`] mode.
        selected: EventId,
    },
    Single {
        /// What event is currently viewed.
        current: EventId,
    },
}
