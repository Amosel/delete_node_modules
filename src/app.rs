pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::{DirEntryItem, DirEntryItemList};
use crate::event::{DirDeleteProcess, DirEntryProcess};
use crate::list::{AsyncContent, Filterable, StatefulList};
use std::error;

#[derive(Debug, Clone)]
pub enum GroupSelection {
    All,
    None,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list: StatefulList<DirEntryItem>,
    pub scanning: bool,
    pub deleting: usize,
    pub search: bool,
    pub group_selection: Option<GroupSelection>,
    pub filter_input: Option<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            list: StatefulList::new_empty(),
            scanning: true,
            deleting: 0,
            search: false,
            filter_input: None,
            group_selection: None,
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

    pub fn toggle_selected_item(&mut self) {
        self.list.toggle_selected_item();
        self.group_selection = None;
    }

    pub fn start_search_entry(&mut self) {
        self.search = true;
    }

    pub fn end_search_entry(&mut self) {
        self.search = false;
    }

    pub fn set_on_and_next(&mut self) {
        self.list.set_on_and_next();
        self.group_selection = None;
    }

    pub fn set_off_and_next(&mut self) {
        self.list.set_off_and_next();
        self.group_selection = None;
    }

    pub fn next(&mut self) {
        self.list.next();
    }

    pub fn previous(&mut self) {
        self.list.previous();
    }

    pub fn toggle_group_selection(&mut self) {
        let group_selection = self
            .group_selection
            .as_ref()
            .map(|group_selection| match group_selection {
                GroupSelection::All => GroupSelection::None,
                GroupSelection::None => {
                    if self.list.visible_items().any(|item| item.is_on) {
                        GroupSelection::None
                    } else {
                        GroupSelection::All
                    }
                }
            })
            .or(Some(GroupSelection::All));

        self.group_selection = group_selection;
    }

    pub fn handle_entry(&mut self, d: DirEntryProcess) {
        match d {
            DirEntryProcess::Started => self.list.set_scanning(),
            DirEntryProcess::Finished => self.list.set_done_scanning(),
            DirEntryProcess::Found(e, size) => self.list.push(DirEntryItem::from_entry(e, size)),
        }
    }
    pub fn handle_delete(&mut self, d: DirDeleteProcess) {
        match d {
            DirDeleteProcess::BatchStarted(d) => self.deleting = d,
            DirDeleteProcess::BatchFinished(_) => {}
            DirDeleteProcess::Deleting(e) => {
                self.list.set_deleting(e);
            }
            DirDeleteProcess::Deleted(e) => {
                self.deleting -= 1;
                self.list.delete(e);
            }
            DirDeleteProcess::Failed(e, error_message) => {
                self.list.set_failed(e, error_message);
                self.deleting -= 1;
            }
        }
    }

    pub fn items_to_delete(&self) -> Vec<&DirEntryItem> {
        if let Some(group_selection) = self.group_selection.as_ref() {
            match group_selection {
                GroupSelection::All => return self.list.visible_items().collect(),
                GroupSelection::None => return vec![],
            }
        }
        self.list
            .visible_items()
            .filter(|item| item.is_on)
            .collect()
    }

    pub fn append_filter_input(&mut self, c: char) {
        self.filter_input
            .get_or_insert_with(Default::default)
            .push(c);
    }
    pub fn delete_filter_input(&mut self) {
        if let Some(filter_input) = &mut self.filter_input {
            filter_input.pop();
            if filter_input.is_empty() {
                // Optional: Reset to None if the string becomes empty
                self.filter_input = None;
                self.search = false;
            }
        }
    }
}
