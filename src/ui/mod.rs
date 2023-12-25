use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::Result;
use itertools::intersperse;
use ratatui::{prelude::*, widgets::*};

use crate::{
    state::{
        schedule,
        store::{State, View},
    },
    Action,
};

pub struct Ui {
    /// The [`Option`] is needed since the [`Ui::draw`] method takes it out for a short time, so it can pass down the [`Ui`] mutably.
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
}

impl Ui {
    pub fn new() -> Result<Self> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Self {
            terminal: Some(terminal),
        })
    }

    pub fn clean_up(self) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn frame(&mut self, state: &State) -> Result<Option<Action>> {
        self.draw(state)?;
        self.input()
    }

    fn draw(&mut self, state: &State) -> Result<()> {
        let mut terminal = self.terminal.take().unwrap();

        terminal.draw(|frame| match state.view {
            View::GridOverview { .. } => self.draw_grid_overview(frame, state),
            View::SingleDetails { .. } => self.draw_single_details(frame, state),
        })?;

        self.terminal = Some(terminal);

        Ok(())
    }

    fn draw_grid_overview(&mut self, _frame: &mut Frame<'_>, _state: &State) {
        todo!()
    }

    fn draw_single_details(&mut self, frame: &mut Frame<'_>, state: &State) {
        let View::SingleDetails { ref current } = state.view else {
            panic!("SingleDetails view draw impl got called without being SingleDetails view")
        };
        let schedule_event = state.schedule.resolve_event(current);

        let area = frame.size();
        draw_metadata_row(area, schedule_event, frame, state)
    }

    fn input(&mut self) -> Result<Option<Action>> {
        const FRAME_DURATION: Duration = Duration::from_millis(16);

        if !event::poll(FRAME_DURATION)? {
            return Ok(None);
        }

        let action = match event::read()? {
            Event::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char('q'),
                ..
            }) => Action::Exit,
            _ => return Ok(None),
        };

        Ok(Some(action))
    }
}

fn draw_metadata_row(
    container: Rect,
    event: &schedule::Event,
    frame: &mut Frame<'_>,
    state: &State,
) {
    // the top metadata row
    let helper_text = Style::new().dark_gray();
    let position = Line::from(vec![
        Span::styled("In ", helper_text),
        Span::raw(event.room.as_str()),
    ]);

    // the individual persons should be concatenated with commas in-between
    // but the last comma should actually be "and" instead
    let last_comma_idx = (event.persons.len() * 2).checked_sub(3);
    let persons = event
        .persons
        .iter()
        .map(|id| state.schedule.resolve_person(id).name.as_str())
        .map(|name| Span::raw(name));
    let persons = intersperse(persons, Span::styled(", ", helper_text))
        .enumerate()
        .map(|(idx, part)| match last_comma_idx {
            Some(last_comma_idx) if last_comma_idx == idx => Span::styled(" and ", helper_text),
            _ => part,
        })
        .collect::<Vec<_>>();
    let persons = Line::from(persons);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(position.width() as u16), Constraint::Min(0)])
        .split(container);

    frame.render_widget(Paragraph::new(position), layout[0]);
    frame.render_widget(
        Paragraph::new(persons).alignment(Alignment::Right),
        layout[1],
    );
}
