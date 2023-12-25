pub mod schedule;
pub mod store;

use eyre::Result;

use crate::Action;
use store::Store;

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
