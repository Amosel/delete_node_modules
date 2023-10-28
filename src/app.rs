pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::DirEntryItem;
use crate::list::StatefulList;
use std::error;

#[derive(Debug)]
pub enum GroupSelection {
    Selected,
    Deselected,
    None
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list: StatefulList<DirEntryItem>,
    pub loading: bool,
    pub group_selection: GroupSelection,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            list: StatefulList::new_empty(),
            loading: true,
            group_selection: GroupSelection::None,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn push(&mut self, item: DirEntryItem) {
        self.list.push(item);
    }

    pub fn toggle(&mut self) {
        self.list.toggle();
    }
}
