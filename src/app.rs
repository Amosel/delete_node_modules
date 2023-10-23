pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::item::Item;
use crate::list::StatefulList;
use std::error;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list: StatefulList<Item>,
    pub loading: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
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

    pub fn toggle(&mut self) {
        self.list.toggle();
    }
}
