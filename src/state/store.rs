use eyre::{Context, ContextCompat, Result};

use crate::{config::Config, DateTime, VerticalDirection};

use super::{
    schedule::TimeCoord,
    schedule::{self, Schedule},
    Action, Update,
};

pub struct Store {
    state: State,
}

pub struct State {
    pub schedule: Schedule,
    pub mode: Mode,
    pub selection: TimeCoord,
    pub grid_state: GridState,
    pub single_state: SingleState,
}

impl Store {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            state: State::new(config)?,
        })
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Update for Store {
    fn update(&mut self, action: Action) {
        self.state.update(action)
    }
}

impl State {
    pub fn new(config: &Config) -> Result<Self> {
        let schedule =
            Schedule::from_xml_file(&config.schedule).context("schedule construction failure")?;

        let first_event = schedule
            .first()
            .context("schedule is empty, nothing to display")?;
        let selection = TimeCoord {
            row: first_event.start,
            idx: 0,
        };

        let grid_state = GridState {
            scroll_at: first_event.start,
        };
        let single_state = SingleState { scroll_at: 0 };

        let mode = Mode::Single;

        Ok(Self {
            schedule,
            mode,
            selection,
            grid_state,
            single_state,
        })
    }

    /// Returns the currently selected event.
    /// Grid and single mode are synchronized in this regard — if one moves its selection, the
    /// other one does, too.
    pub fn selected_event(&self) -> &schedule::Event {
        &self.schedule[&self.selection]
    }
}

impl Update for State {
    fn update(&mut self, action: Action) {
        // generally we can forward all actions
        // except for scrolling, which is only relevant for the mode the user is currently observing
        if matches!(action, Action::Scroll(_)) {
            match self.mode {
                Mode::Grid => self.grid_state.update(action),
                Mode::Single => self.single_state.update(action),
            }
            return;
        }

        self.grid_state.update(action);
        self.single_state.update(action);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    /// Overview over all events and their chronological order.
    Grid,
    /// One event in all detail.
    Single,
}

#[derive(Copy, Clone, Debug)]
pub struct GridState {
    /// Topmost point in time of where the scroll currently is.
    pub scroll_at: DateTime,
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
