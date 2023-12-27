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

const COLUMNS: usize = 7;

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
    timeline: BTreeMap<DateTime, SlottedVec<schedule::EventId, COLUMNS>>,
}

impl ScheduleGrid {
    fn new(base: &schedule::Schedule) -> Self {
        let mut active_events = SlottedVec::<_, COLUMNS>::new();
        let mut grid = Self::default();

        for (now, just_starting) in base.time_map() {
            // cull inactive events
            active_events.retain(|(_, end)| &*end > now);

            // insert all new ones
            active_events.extend(just_starting.into_iter().map(|event| {
                let event = &base[event];
                (event.id, event.end())
            }));

            // note down anything that's active now
            grid.timeline.insert(
                *now,
                active_events
                    .iter()
                    .map(|slot| slot.map(|(event, _end)| event.clone()))
                    .collect(),
            );
        }

        grid
    }

    fn render(&self, state: &State, frame: &mut Frame<'_>) {
        let mut widths = vec![Constraint::Length(17)];
        widths.extend(iter::repeat(Constraint::Ratio(1, 9)).take(COLUMNS));

        let cell_width = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(&widths)
            .split(frame.size())[1]
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
                    // TODO: extract this into its own function
                    events.iter().map(|id| {
                        let Some(id) = id else {
                            return Cell::new("");
                        };

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

        let mut table_state = TableState::new();

        frame.render_stateful_widget(Table::new(rows, widths), frame.size(), &mut table_state);
    }
}

/// [`Vec`], but fixed to a compile-time size and keeping elements at the same position regardless
/// of elements removed in before.
#[derive(Debug)]
struct SlottedVec<T, const N: usize> {
    data: [Option<T>; N],
}

impl<T, const N: usize> SlottedVec<T, N> {
    /// [`None`] but const so it can be used as a copied expression in [`Default`].
    const NONE: Option<T> = None;

    fn new() -> Self {
        Self::default()
    }

    /// Runs through all contained elements and removes them if the predicate returns [`false`].
    fn retain(&mut self, mut predicate: impl FnMut(&mut T) -> bool) {
        for slot in &mut self.data {
            let Some(item) = slot else { continue };

            if !(predicate)(item) {
                *slot = Self::NONE;
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Option<T>> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> {
        self.data.iter_mut()
    }
}

impl<T, const N: usize> Default for SlottedVec<T, N> {
    fn default() -> Self {
        Self {
            data: [Self::NONE; N],
        }
    }
}

impl<T, const N: usize> Extend<T> for SlottedVec<T, N> {
    /// Try to fill the vector up from the start.
    ///
    /// Once the vector is all filled up, the iterator is dropped, without pulling remaining elements.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // a bit like k-merge
        // we just iterate through both the vec and the iter
        let mut iter = iter.into_iter();
        let target = self.iter_mut();
        for slot in target {
            if slot.is_some() {
                continue;
            }

            // oh noice, we have a free slot, let's fill it
            let item = iter.next();
            if item.is_none() {
                // oh no the target iter is empty qwq
                // technically we don't know if the iterator is fused
                // so we could just continue here? idk
                return;
            }
            *slot = item;
        }
    }
}

impl<T, const N: usize> FromIterator<Option<T>> for SlottedVec<T, N> {
    fn from_iter<I: IntoIterator<Item = Option<T>>>(iter: I) -> Self {
        let mut vec = Self::default();

        for (slot, item) in vec.iter_mut().zip(iter) {
            *slot = item;
        }

        vec
    }
}
