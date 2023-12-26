//! Overview over all events in a schedule.

use ratatui::{prelude::*, widgets::*};

use crate::state::store::{GridState, State};

use super::TerminalEvent;

pub struct View<'state> {
    pub state: &'state State,
    pub mode_state: &'state GridState,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, _frame: &mut Frame<'_>) {
        todo!()
    }

    fn process(&mut self, _event: TerminalEvent) -> Option<crate::Action> {
        todo!()
    }
}
