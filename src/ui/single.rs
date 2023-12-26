//! One specific event with all its gory details, presented like the first page of a paper.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use itertools::intersperse;
use ratatui::{prelude::*, widgets::*};
use time::UtcOffset;

use crate::{
    state::{
        schedule,
        store::{Mode, State},
    },
    Action, DateTime, VerticalDirection,
};

use super::{helper_span, TerminalEvent, DATETIME_FORMAT_LONG, DATETIME_FORMAT_SHORT};

pub struct View<'state> {
    pub state: &'state State,
}

impl<'state> super::View for View<'state> {
    fn draw(&mut self, frame: &mut Frame<'_>) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 4), Constraint::Min(0)])
            .split(frame.size());

        let event = self.state.selected_event();
        let mut render = RenderState {
            view: self,
            event,
            frame,
        };

        render.metadata(layout[0]);
        render.content(layout[1]);
    }

    fn process(&mut self, event: super::TerminalEvent) -> Option<crate::Action> {
        let action = match event {
            TerminalEvent::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => match code {
                KeyCode::Esc | KeyCode::Enter => Action::SwitchTo(Mode::Grid),
                KeyCode::Char('k') => Action::Scroll(VerticalDirection::Up),
                KeyCode::Char('j') => Action::Scroll(VerticalDirection::Down),
                _ => return None,
            },
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
    fn metadata(&mut self, container: Rect) {
        // the short format with only the time is ideal when the event is today
        // the long format should be displayed otherwise
        // that check is done for start/end individually
        let now = DateTime::now_utc();

        let [start, end]: [Span; 2] = [self.event.start, self.event.end()]
            .into_iter()
            .map(|point| {
                let is_today = now.date() == point.to_offset(UtcOffset::UTC).date();
                let format = match is_today {
                    true => DATETIME_FORMAT_SHORT,
                    false => DATETIME_FORMAT_LONG,
                };
                let point = point.format(format).unwrap();
                Span::raw(point)
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let duration = humantime::Duration::from(self.event.duration.unsigned_abs());
        let duration = Span::raw(duration.to_string());

        let vert_layout = Layout::default()
            .constraints([Constraint::Length(4), Constraint::Min(0)])
            .split(container);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(7), Constraint::Min(0)])
            .split(vert_layout[1]);

        self.frame.render_widget(
            Paragraph::new(
                ["where", "when", "+", "=", "", "track", "type"]
                    .into_iter()
                    .map(|label| vec![helper_span(label), Span::raw(" ")])
                    .map(Line::from)
                    .collect::<Vec<_>>(),
            )
            .alignment(Alignment::Right),
            layout[0],
        );
        self.frame.render_widget(
            Paragraph::new(
                [
                    Span::raw(self.event.room.as_str()),
                    start,
                    duration,
                    end,
                    Span::raw(""),
                    Span::raw(self.event.track.as_str()),
                    Span::raw(self.event.r#type.as_str()),
                ]
                .into_iter()
                .map(Line::from)
                .collect::<Vec<_>>(),
            ),
            layout[1],
        );
    }

    fn content(&mut self, container: Rect) {
        let layout = Layout::default()
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .margin(1)
            .split(container);
        self.header(layout[0]);
        self.text(layout[1]);
    }

    fn header(&mut self, container: Rect) {
        let title = Span::raw(&self.event.title).bold();
        let subtitle = Span::raw(&self.event.subtitle).italic();

        // the individual persons should be concatenated with commas in-between
        // but the last comma should actually be "and" instead
        let last_comma_idx = (self.event.persons.len() * 2).checked_sub(3);
        let persons = self
            .event
            .persons
            .iter()
            .map(|id| self.view.state.schedule[id].name.as_str())
            .map(|name| Span::raw(name));
        let mut persons = intersperse(persons, helper_span(", "))
            .enumerate()
            .map(|(idx, part)| match last_comma_idx {
                Some(last_comma_idx) if last_comma_idx == idx => helper_span(" and "),
                _ => part,
            })
            .collect::<Vec<_>>();
        persons.insert(0, helper_span("by "));

        let lines = vec![
            Line::from(title),
            Line::from(subtitle),
            Line::raw(""),
            Line::from(persons),
        ];
        self.frame.render_widget(
            Paragraph::new(lines).alignment(Alignment::Center),
            container,
        );
    }

    fn text(&mut self, container: Rect) {
        // ratatui seems to perform no wrapping on its own
        // so let's use the textwrap crate instead

        let wrap = |content| super::wrap(content, container.width as usize);

        let mut text = Text::from(helper_span("abstract"));
        text.extend(wrap(&self.event.r#abstract));
        text.extend([Span::raw(""), helper_span("description")]);
        text.extend(wrap(&self.event.description));

        let paragraph = Paragraph::new(text).scroll((self.view.state.single_state.scroll_at, 0));

        self.frame.render_widget(paragraph, container);
    }
}
