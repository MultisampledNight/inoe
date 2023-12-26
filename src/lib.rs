//! I don't even know anymore what I'm doing, I think.
//!
//! # Architecture
//!
//! A bit of a mix between the [Component architecture] and the [Flux architecture].
//!
//! Essentially two-split into
//!
//! - [`state`]: Cares about application logic and state, also knowing which pane for
//!   example the user is currently in through the [`state::store::Mode`] enum.
//!
//!   It doesn't have a concept of a "frame" or the like, instead, all modification happens through
//!   the [`Action`] enum, which acts as a message from the UI to the state.
//!
//! - [`ui`]: Cares about drawing things each frame, and getting input to convert it into
//!   [`Action`]s. For this, the [`ratatui`] and [`crossterm`] crates are leveraged in
//!   immediate-mode style.
//!
//!   While [`ui::Ui`] is held during the whole program lifetime, it holds only things like
//!   terminal handles or caches. It creates every frame a new [`ui::View`] for the current
//!   [`state::store::Mode`], drawing it, fetching it for input and destroying it right again.
//!
//! [Component architecture]: https://ratatui.rs/concepts/application-patterns/component-architecture/
//! [Flux architecture]: https://ratatui.rs/concepts/application-patterns/flux-architecture/

pub mod config;
pub mod state;
pub mod ui;

use eyre::{Context, Result};
use state::{store::Mode, Dispatcher};
use ui::Ui;

pub type DateTime = time::OffsetDateTime;

pub fn run() -> Result<()> {
    let app = App::new()?;
    app.run()
}

pub struct App {
    ui: Ui,
    dispatcher: Dispatcher,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = config::parse();
        let dispatcher = Dispatcher::new(&config)?;
        let ui = Ui::new().context("ui creation failure")?;

        // could store config in app if needed
        Ok(Self { ui, dispatcher })
    }

    pub fn run(mut self) -> Result<()> {
        loop {
            let state = self.dispatcher.store.state();
            let action = self.ui.frame(state)?;

            if let Some(action) = action {
                let should_exit = matches!(action, Action::Exit);
                self.dispatcher.dispatch(action);

                if should_exit {
                    break;
                }
            }
        }

        self.ui.clean_up()?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Exit,
    Select(To),
    SwitchTo(Mode),
    Scroll(VerticalDirection),
}

/// Direction but not since the "direction" is taken by ratatui already.
#[derive(Copy, Clone, Debug)]
pub enum To {
    Left,
    Right,
    Up,
    Below,
}

#[derive(Copy, Clone, Debug)]
pub enum VerticalDirection {
    Down,
    Up,
}
