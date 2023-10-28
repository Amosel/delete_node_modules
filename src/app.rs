pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::DirEntryItem;
use crate::list::StatefulList;
use std::{error, vec};

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
    pub fn total_selected_size_mb(&self) -> u64 {
        match self.group_selection {
            GroupSelection::Selected => self
                .list
                .items
                .iter()
                .map(|item| item.size().unwrap())
                .sum(),
            GroupSelection::Deselected => 0,
            GroupSelection::None => self
                .list
                .items
                .iter()
                .filter(|item| item.is_on)
                .map(|item| item.size().unwrap())
                .sum(),
        }
    }
    pub fn total_selected(&self) -> Vec<DirEntryItem> {
        match self.group_selection {
            GroupSelection::Selected => self.list.items.clone(),
            GroupSelection::Deselected => vec![],
            GroupSelection::None => self
                .list
                .items
                .iter()
                .filter(|item| item.is_on)
                .cloned() // If DirEntryItem implements Clone
                .collect(),
        }
    }
    pub fn total_selected_count(&self) -> usize {
        match self.group_selection {
            GroupSelection::Deselected => 0,
            GroupSelection::Selected => self.list.items.len(),
            GroupSelection::None => self.list.items.iter().filter(|item| item.is_on).count(),
        }
    }
}
