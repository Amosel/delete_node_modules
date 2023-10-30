pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::DirEntryItem;
use crate::list::StatefulList;
use std::error;

#[derive(Debug)]
pub enum GroupSelection {
    Selected,
    Deselected,
    None,
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
            group_selection: GroupSelection::Selected,
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
        self.group_selection = GroupSelection::None;
    }

    pub fn set_on_and_next(&mut self) {
        self.list.set_on_and_next();
        self.group_selection = GroupSelection::None;
    }

    pub fn set_off_and_next(&mut self) {
        self.list.set_off_and_next();
        self.group_selection = GroupSelection::None;
    }

    pub fn next(&mut self) {
        self.list.next();
    }

    pub fn previous(&mut self) {
        self.list.previous();
    }

    pub fn toggle_group(&mut self) {
        self.group_selection = match self.group_selection {
            GroupSelection::Selected => GroupSelection::Deselected,
            GroupSelection::Deselected => {
                if self.list.items.iter().any(|item| item.is_on) {
                    GroupSelection::None
                } else {
                    GroupSelection::Selected
                }
            }
            GroupSelection::None => GroupSelection::Selected,
        }
    }
}
