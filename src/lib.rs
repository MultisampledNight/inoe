pub mod state;
pub mod ui;

use eyre::{Context, Result};
use state::Dispatcher;
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
        let ui = Ui::new().context("ui creation failure")?;
        let dispatcher = Dispatcher::new()?;

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

pub enum Action {
    Exit,
}
