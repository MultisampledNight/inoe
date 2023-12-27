use eyre::{Context, ContextCompat, Result};
use ratatui::layout::Direction;

use crate::{config::Config, DateTime, To, VerticalDirection};

use super::{
    schedule::TimeCoord,
    schedule::{self, Schedule},
    Action, Update,
};

pub struct Store {
    state: State,
}

pub struct State {
    /// The actual bare data we want to display.
    pub schedule: Schedule,

    /// What mode the user is currently looking at.
    pub mode: Mode,

    /// What event is selected at the moment, and where to find it.
    pub selection: TimeCoord,

    /// State specific to the grid mode.
    pub grid_state: GridState,
    /// State specific to the single/detail mode.
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

        Ok(Self {
            schedule,
            mode: Mode::default(),
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

    fn scroll(&mut self, direction: Direction, amount: isize) {
        if amount == 0 {
            // valid, but no effect
            return;
        }

        let current_line = self
            .schedule
            .time_map()
            .get(&self.selection.row)
            .expect("selected row to be valid");

        match direction {
            Direction::Horizontal => {
                let new_idx = self.selection.idx as isize + amount;

                if (0..current_line.len() as isize).contains(&new_idx) {
                    // still in the line! just set it
                    self.selection.idx = new_idx as usize;
                    return;
                }

                // oh well, guess we need to adjust the line
                let row_diff = new_idx.signum();
                let target = self.schedule.relative(row_diff, self.selection.row);

                let Some((target_row, target_events)) = target else {
                    // oh no, out of range! let's just stay where we are then
                    return;
                };

                // otherwise we're good! let's set it
                self.selection.row = *target_row;
                self.selection.idx = match row_diff {
                    1 => 0,
                    -1 => target_events.len() - 1,
                    _ => unreachable!(),
                }
            }
            Direction::Vertical => todo!(),
        }
    }
}

impl Update for State {
    fn update(&mut self, action: Action) {
        // generally we can forward all actions

        // except for
        match action {
            // scrolling, which is only relevant for the mode the user is currently observing
            Action::Scroll(_) => match self.mode {
                Mode::Grid => self.grid_state.update(action),
                Mode::Single => self.single_state.update(action),
            },
            // switching modes
            Action::SwitchTo(new_mode) => {
                self.mode = new_mode;
            }
            // changing event selection
            Action::Select(dir) => match dir {
                To::Left => self.scroll(Direction::Horizontal, -1),
                To::Right => self.scroll(Direction::Horizontal, 1),
                To::Up => self.scroll(Direction::Vertical, -1),
                To::Below => self.scroll(Direction::Vertical, 1),
            },
            // otherwise, just tell both about it
            _ => {
                self.grid_state.update(action);
                self.single_state.update(action);
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum Mode {
    /// Overview over all events and their chronological order.
    #[default]
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
