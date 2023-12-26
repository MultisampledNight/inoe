//! Overview over all events in a schedule.

use std::{collections::BTreeMap, iter};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};

use crate::{
    state::{
        schedule,
        store::{Mode, State},
    },
    Action, DateTime, To,
};

use super::{wrap, TerminalEvent, DATETIME_FORMAT_LONG};

pub struct View<'state> {
    pub state: &'state State,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, frame: &mut Frame<'_>) {
        let grid = ScheduleGrid::new(&self.state.schedule);
        grid.render(&self.state, frame);
    }

    fn process(&mut self, event: super::TerminalEvent) -> Option<crate::Action> {
        let action = match event {
            TerminalEvent::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Enter => Action::SwitchTo(Mode::Single),
                KeyCode::Char('k') => Action::Select(To::Up),
                KeyCode::Char('j') => Action::Select(To::Below),
                _ => return None,
            },
            _ => return None,
        };

        Some(action)
    }
}

/// A fully "simulated" schedule, where each timeslot is assigned.
///
/// **Note:** The current implementation for this is horribly inefficient and has a runtime of
/// _O_(_nm_), where _n_ is the count of events and _m_ the number of concurrently occuring events.
/// I'm sure there's some better and smarter way to do this implicitly and still create a table,
/// but this works for the moment.
#[derive(Debug, Default)]
pub struct ScheduleGrid {
    timeline: BTreeMap<DateTime, Vec<schedule::EventId>>,
}

impl ScheduleGrid {
    fn new(base: &schedule::Schedule) -> Self {
        let mut active_events = BTreeMap::new();
        let mut grid = Self::default();

        for (now, just_starting) in base.time_map() {
            // cull inactive events
            active_events.retain(|_, end| &*end > now);

            // insert all new ones
            active_events.extend(just_starting.into_iter().map(|event| {
                let event = &base[event];
                (event.id, event.end())
            }));

            // note down anything that's active now
            grid.timeline
                .insert(*now, active_events.keys().cloned().collect());
        }

        grid
    }

    fn render(&self, state: &State, frame: &mut Frame<'_>) {
        let cell_width = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 6), Constraint::Min(0)])
            .split(frame.size())[0]
            .width;

        // fetch only the relevant part of the timeline
        // rendering the *whole* timeline would be far too laggy
        let relevant_timeline = self
            .timeline
            .range(state.grid_state.scroll_at..)
            .take(usize::from(frame.size().height / 3 + 1));

        let selected = state.selected_event();

        let rows = relevant_timeline
            .map(|(timestamp, events)| {
                iter::once(Cell::new(timestamp.format(DATETIME_FORMAT_LONG).unwrap())).chain(
                    events.iter().map(|id| {
                        let text = state.schedule[id].title.as_str();
                        let cell = Cell::new(wrap(text, cell_width as usize).collect::<Vec<_>>());
                        if selected.id == *id {
                            cell.reversed()
                        } else {
                            cell
                        }
                    }),
                )
            })
            .map(|cells| Row::new(cells).height(3));

        let widths = [
            Constraint::Length(17),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
            Constraint::Length(cell_width),
        ];

        let mut table_state = TableState::new();

        frame.render_stateful_widget(Table::new(rows, widths), frame.size(), &mut table_state);
    }
}
