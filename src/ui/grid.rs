//! Overview over all events in a schedule.

use ratatui::{prelude::*, widgets::*};

use crate::state::store::State;

pub struct View<'state> {
    pub state: &'state State,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, _frame: &mut Frame<'_>) {
        todo!()
    }
}
