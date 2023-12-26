//! Overview over all events in a schedule.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use crate::{
    state::store::{Mode, State},
    Action,
};

use super::TerminalEvent;

pub struct View<'state> {
    pub state: &'state State,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, frame: &mut Frame<'_>) {
        let rows = vec![
            Row::new(vec!["Yeehaw"]),
            Row::new(vec!["this is professional", "i think"]),
            Row::new(vec!["well it's accurate", "anyway", "E", "E"]),
        ];

        let widths = [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">");

        let mut table_state = TableState::default();
        frame.render_stateful_widget(table, frame.size(), &mut table_state);
    }

    fn process(&mut self, event: super::TerminalEvent) -> Option<crate::Action> {
        let action = match event {
            TerminalEvent::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => Action::SwitchTo(Mode::Single),
            _ => return None,
        };

        Some(action)
    }
}
