//! One specific event with all its gory details, presented like the first page of a paper..

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use itertools::intersperse;
use ratatui::{prelude::*, widgets::*};
use time::{macros::format_description, UtcOffset};

use crate::{
    state::{
        schedule,
        store::{SingleState, State},
    },
    Action, DateTime,
};

use super::{helper_span, TerminalEvent};

pub struct View<'state> {
    pub state: &'state State,
    pub mode_state: &'state SingleState,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, frame: &mut Frame<'_>) {
        let layout = Layout::default()
            .constraints([
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(frame.size());

        let event = self.state.schedule.resolve_event(&self.mode_state.current);
        let mut render = RenderState {
            view: self,
            event,
            frame,
        };

        render.metadata_row(layout[0]);
        render.header(layout[1]);
        render.timeframe(layout[2]);
    }

    fn process(&mut self, event: super::TerminalEvent) -> Option<crate::Action> {
        let action = match event {
            // TODO: should actually go to the grid view later on
            TerminalEvent::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('q'),
                ..
            }) => Action::Exit,
            _ => return None,
        };

        Some(action)
    }
}

struct RenderState<'view, 'state, 'frame, 'life> {
    view: &'view View<'state>,
    event: &'state schedule::Event,
    frame: &'frame mut Frame<'life>,
}

impl<'view, 'state, 'frame, 'life> RenderState<'view, 'state, 'frame, 'life> {
    fn metadata_row(&mut self, container: Rect) {
        // the top metadata row
        let position = Line::from(vec![
            helper_span("In "),
            Span::raw(self.event.room.as_str()),
        ]);

        // the individual persons should be concatenated with commas in-between
        // but the last comma should actually be "and" instead
        let last_comma_idx = (self.event.persons.len() * 2).checked_sub(3);
        let persons = self
            .event
            .persons
            .iter()
            .map(|id| self.view.state.schedule.resolve_person(id).name.as_str())
            .map(|name| Span::raw(name));
        let persons = intersperse(persons, helper_span(", "))
            .enumerate()
            .map(|(idx, part)| match last_comma_idx {
                Some(last_comma_idx) if last_comma_idx == idx => helper_span(" and "),
                _ => part,
            })
            .collect::<Vec<_>>();
        let persons = Line::from(persons);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(position.width() as u16), Constraint::Min(0)])
            .split(container);

        self.frame
            .render_widget(Paragraph::new(position), layout[0]);
        self.frame.render_widget(
            Paragraph::new(persons).alignment(Alignment::Right),
            layout[1],
        );
    }

    fn header(&mut self, container: Rect) {
        let title = Span::raw(&self.event.title).bold();
        let subtitle = Span::raw(&self.event.subtitle).italic();
        let lines = vec![Line::from(title), Line::from(subtitle)];
        self.frame.render_widget(
            Paragraph::new(lines).alignment(Alignment::Center),
            container,
        );
    }

    fn timeframe(&mut self, container: Rect) {
        // the short format with only the time is ideal when the event is today
        // the long format should be displayed otherwise
        // that check is done for start/end individually
        let short_format = format_description!("[hour] [minute]");
        let long_format = format_description!("[year]-[month]-[day]  [hour] [minute]");
        let now = DateTime::now_utc();

        let [start, end]: [Span; 2] = [self.event.start, self.event.end()]
            .into_iter()
            .map(|point| {
                let is_today = now.date() == point.to_offset(UtcOffset::UTC).date();
                let format = match is_today {
                    true => short_format,
                    false => long_format,
                };
                let point = point.format(format).unwrap();
                Span::raw(point)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let duration = humantime::Duration::from(self.event.duration.unsigned_abs());
        let duration = Span::raw(duration.to_string());

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(container);

        self.frame.render_widget(
            Paragraph::new(vec![
                Line::from(vec![helper_span("start "), start]),
                Line::from(vec![helper_span("= "), end]),
            ])
            .alignment(Alignment::Right),
            layout[0],
        );
        self.frame.render_widget(
            Paragraph::new(vec![Line::from(vec![helper_span(" + "), duration])]),
            layout[1],
        );
    }
}
