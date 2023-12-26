use eyre::{Context, ContextCompat, Result};

use crate::{DateTime, VerticalDirection};

use super::{schedule::EventId, schedule::Schedule, Action, Update};

pub struct Store {
    state: State,
}

pub struct State {
    pub schedule: Schedule,
    pub mode: Mode,
}

impl Store {
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: State::new()?,
        })
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Update for Store {
    fn update(&mut self, action: Action) {
        self.state.mode.update(action);
    }
}

impl State {
    pub fn new() -> Result<Self> {
        let schedule = Schedule::new().context("schedule construction failure")?;
        let first_event = schedule
            .first()
            .context("schedule is empty, nothing to display")?;
        let view = Mode::Single(SingleState {
            current: first_event.id,
            scroll_at: 0,
        });

        Ok(Self {
            schedule,
            mode: view,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Grid(GridState),
    Single(SingleState),
}

impl Update for Mode {
    fn update(&mut self, action: Action) {
        match self {
            Self::Grid(state) => state.update(action),
            Self::Single(state) => state.update(action),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GridState {
    /// Topmost point of where the scroll currently is.
    pub scroll_at: DateTime,
    /// What event is currently selected and would be viewed if switched into [`Mode::Single`].
    pub selected: EventId,
}

impl Update for GridState {
    fn update(&mut self, action: Action) {
        match action {
            _ => (),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SingleState {
    /// Topmost line of where the scroll currently is.
    pub scroll_at: u16,
    /// What event is currently being viewed.
    pub current: EventId,
}

impl Update for SingleState {
    fn update(&mut self, action: Action) {
        match action {
            Action::Scroll(VerticalDirection::Down) => {
                self.scroll_at = self.scroll_at.saturating_add(1)
            }
            Action::Scroll(VerticalDirection::Up) => {
                self.scroll_at = self.scroll_at.saturating_sub(1)
            }
            _ => (),
        }
    }
}
