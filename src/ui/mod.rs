use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, EventStream, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::{Result};
use ratatui::{prelude::*, widgets::*};

use crate::Action;

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    events: EventStream,
}

impl Ui {
    pub fn new() -> Result<Self> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(Self {
            terminal,
            events: EventStream::new(),
        })
    }

    pub fn clean_up(self) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn frame(&mut self) -> Result<Option<Action>> {
        self.draw()?;
        self.input()
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(Paragraph::new("yeehaw! press q to quit"), area)
        })?;

        Ok(())
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
