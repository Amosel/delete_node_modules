pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::DirEntryItem;
use crate::event::{DirDeleteProcess, DirEntryProcess};
use crate::list::{AsyncContent, Filterable, StatefulList};
use std::error;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum GroupSelection {
    All,
    None,
}

#[derive(Debug)]
pub struct Deletes {
    pub history: Option<Vec<(PathBuf, u64)>>,
    pub queued: Option<Vec<(PathBuf, u64)>>,
    pub current: Option<Vec<(PathBuf, u64)>>,
    pub failed: Option<Vec<(PathBuf, u64, String)>>,
}

impl Deletes {
    pub fn add_to_queue(&mut self, item: (PathBuf, u64)) {
        self.queued.get_or_insert_with(Vec::new).push(item);
    }
    // A helper function to reduce repetition
    fn move_item_to_vec(
        item: (PathBuf, u64),
        from: &mut Option<Vec<(PathBuf, u64)>>,
        to: &mut Option<Vec<(PathBuf, u64)>>,
    ) {
        if let Some(vec) = from {
            vec.retain(|i| *i != item);
        }
        to.get_or_insert_with(Vec::new).push(item);
    }

    pub fn add_to_current(&mut self, item: (PathBuf, u64)) {
        Self::move_item_to_vec(item, &mut self.queued, &mut self.current);
    }

    pub fn add_to_history(&mut self, item: (PathBuf, u64)) {
        Self::move_item_to_vec(item, &mut self.current, &mut self.history);
    }

    pub fn add_to_failed(&mut self, item: (PathBuf, u64), error_message: String) {
        if let Some(vec) = self.current.as_mut() {
            vec.retain(|i| *i == item);
        }
        self.failed
            .get_or_insert_with(Vec::new)
            .push((item.0, item.1, error_message))
    }
    pub fn active(&self) -> Option<(usize, u64)> {
        let queued_size = self
            .queued
            .as_ref()
            .map_or((0, 0), |v| (v.len(), v.iter().map(|(_, size)| size).sum()));
        let current_size = self
            .current
            .as_ref()
            .map_or((0, 0), |v| (v.len(), v.iter().map(|(_, size)| size).sum()));

        if queued_size.0 + current_size.0 == 0 {
            None
        } else {
            Some((
                queued_size.0 + current_size.0,
                queued_size.1 + current_size.1,
            ))
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list: StatefulList<DirEntryItem>,
    pub scanning: bool,
    pub deletes: Deletes,
    pub search: bool,
    pub group_selection: Option<GroupSelection>,
    pub filter_input: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            list: StatefulList::new_empty(),
            scanning: false,
            deletes: Deletes {
                queued: None,
                history: None,
                current: None,
                failed: None,
            },
            search: false,
            filter_input: None,
            group_selection: None,
        }
    }
}
impl App {
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
            DirDeleteProcess::Deleting(item) => {
                self.deletes.add_to_current(item);
            }
            DirDeleteProcess::Deleted(item) => {
                self.deletes.add_to_history(item);
            }
            DirDeleteProcess::Failed(item, error_message) => {
                self.deletes.add_to_failed(item, error_message);
            }
        }
    }

    pub fn items_to_delete(&self) -> Vec<DirEntryItem> {
        if let Some(group_selection) = self.group_selection.as_ref() {
            match group_selection {
                GroupSelection::All => return self.list.visible_items().cloned().collect(),
                GroupSelection::None => return vec![],
            }
        }
        self.list
            .visible_items()
            .filter(|item| item.is_on)
            .cloned()
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
