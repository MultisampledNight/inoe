pub mod schedule;
pub mod store;

use eyre::Result;

use crate::{config::Config, Action};
use store::Store;

pub struct Dispatcher {
    pub store: Store,
}

impl Dispatcher {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            store: Store::new(config)?,
        })
    }

    pub fn dispatch(&mut self, action: Action) {
        self.store.update(action);
    }
}

pub trait Update {
    fn update(&mut self, action: Action);
}
