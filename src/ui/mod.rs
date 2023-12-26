//! UI setup, drawing and layouting logic, as well as event handling.
//!
//! The idea is that for each variant of [`crate::state::store::Mode`], there's one corresponding
//! submodule in this folder, which takes care of drawing one frame in that view and handling input
//! appropiately.
//!
//! All state is actually held in [`crate::state`] by the dispatcher and its store, so the code
//! here only has to draw and find out what actions to send.
//!
//! See the [`crate`] module documentation for details.

mod grid;
mod single;

use std::{
    io::{stdout, Stdout},
    panic,
    time::Duration,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyEventKind, MouseEvent,
        MouseEventKind,
    },
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::Result;
use hyphenation::{Language, Load, Standard};
use ratatui::prelude::*;
use textwrap::{Options, WordSplitter};
use time::{format_description::FormatItem, macros::format_description};

use crate::{
    state::store::{Mode, State},
    Action, To, VerticalDirection,
};

/// Implementation of viewing a specific [`Mode`]. Created for one frame, then destroyed again.
pub trait View {
    /// Draw this mode in all detail.
    fn draw(&mut self, frame: &mut Frame<'_>);

    /// Process mode-specific input [`TerminalEvent`]s.
    fn process(&mut self, _event: TerminalEvent) -> Option<Action> {
        None
    }
}

fn map_mode_to_view<'state>(state: &'state State) -> Box<dyn View + 'state> {
    // could be facilitated with a macro if the manual matching becomes too repetetive
    match state.mode {
        Mode::Grid => Box::new(grid::View { state }),
        Mode::Single => Box::new(single::View { state }),
    }
}

pub struct Ui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Ui {
    pub fn new() -> Result<Self> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        init_terminal(&mut terminal)?;
        install_panic_hook();

        Ok(Self { terminal })
    }

    pub fn clean_up(self) -> Result<()> {
        reset_terminal()?;
        Ok(())
    }

    pub fn frame(&mut self, state: &State) -> Result<Option<Action>> {
        let mut view = map_mode_to_view(state);
        self.draw(&mut view)?;
        self.input(&mut view)
    }

    fn draw<'state>(&mut self, view: &mut Box<dyn View + 'state>) -> Result<()> {
        self.terminal.draw(|frame| view.draw(frame))?;
        Ok(())
    }

    fn input<'state>(&mut self, view: &mut Box<dyn View + 'state>) -> Result<Option<Action>> {
        const FRAME_DURATION: Duration = Duration::from_millis(16);

        if !event::poll(FRAME_DURATION)? {
            return Ok(None);
        }
        let event = event::read()?;

        // try to match against "well-known" ones first
        // so each one doesn't have to handle scrolling again, for example

        let mut forward = |event| Ok(view.process(event));

        let action = match event {
            // TODO: should actually go to the grid view later on
            TerminalEvent::Key(KeyEvent {
                kind: KeyEventKind::Press,
                code: KeyCode::Char(ch),
                ..
            }) => match ch {
                'h' => Action::Select(To::Left),
                'l' => Action::Select(To::Right),
                'q' => Action::Exit,
                _ => return forward(event),
            },
            TerminalEvent::Mouse(MouseEvent { kind, .. }) => match kind {
                MouseEventKind::ScrollUp => Action::Scroll(VerticalDirection::Up),
                MouseEventKind::ScrollDown => Action::Scroll(VerticalDirection::Down),
                _ => return forward(event),
            },
            _ => return forward(event),
        };

        Ok(Some(action))
    }
}

fn init_terminal<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    enable_raw_mode()?;
    stdout()
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;
    terminal.clear()?;

    Ok(())
}

fn install_panic_hook() {
    let original_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }))
}

fn reset_terminal() -> Result<()> {
    stdout()
        .execute(DisableMouseCapture)?
        .execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

pub type TerminalEvent = crossterm::event::Event;

pub const DATETIME_FORMAT_LONG: &'static [FormatItem<'static>] =
    format_description!("[year]-[month]-[day]  [hour] [minute]");
pub const DATETIME_FORMAT_SHORT: &'static [FormatItem<'static>] =
    format_description!("[hour] [minute]");

pub fn helper_span(content: &str) -> Span<'_> {
    Span::styled(content, Style::new().dark_gray())
}

pub fn wrap(content: &str, width: usize) -> impl Iterator<Item = Line> {
    let mut opts = Options::new(width);

    let dictionary =
        Standard::from_embedded(Language::EnglishUS).expect("embedded dict to be correct");
    opts.word_splitter = WordSplitter::Hyphenation(dictionary);

    textwrap::wrap(content, opts).into_iter().map(Line::raw)
}
