pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use std::error;
use crate::item::Item;
use crate::list::StatefulList;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub list: StatefulList<Item>,
    // pub items: Vec<std::path::PathBuf>,
    pub loading: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            counter: 0,
            list: StatefulList::new_empty(),
            loading: true,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn push(&mut self, item: Item) {
        self.list.push(item);
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
