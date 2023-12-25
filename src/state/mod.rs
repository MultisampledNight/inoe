mod schedule;
mod store;

pub use schedule::Schedule;
pub use store::{State, Store};

use eyre::Result;

use crate::Action;

pub struct Dispatcher {
    store: Store,
}

impl Dispatcher {
    pub fn new() -> Result<Self> {
        Ok(Self {
            store: Store::new()?,
        })
    }

    pub fn dispatch(&mut self, action: Action) {
        self.store.update(action);
    }
}
