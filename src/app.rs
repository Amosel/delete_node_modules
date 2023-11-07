pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::actions::ActionState;
use crate::dir_entry_item::DirEntryItem;
use crate::event::{DirDelete, DirSearch};
use crate::list::{Filterable, StatefulList, Toggle};
use std::error;

impl PartialEq for DirEntryItem {
    fn eq(&self, other: &Self) -> bool {
        self.entry.path() == other.entry.path()
    }
}

#[derive(Debug, Clone)]
pub enum GroupSelection {
    All,
    None,
}

#[derive(Debug, Default)]
pub struct Log {
    pub current: ItemCounter,
    pub history: ItemCounter,
    pub failed: ItemCounter,
}

impl Log {
    fn add(&mut self, size: u64) {
        self.current.add(size);
    }
    fn finished(&mut self, size: u64) -> bool {
        if self.current.remove(size) {
            self.history.add(size);
            true
        } else {
            false
        }
    }
    fn failed(&mut self, size: u64) -> bool {
        if self.current.remove(size) {
            self.failed.add(size);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct ItemCounter {
    pub count: usize,
    pub total_size: u64,
}

impl ItemCounter {
    pub fn new() -> Self {
        ItemCounter {
            count: 0,
            total_size: 0,
        }
    }
    // Add an item with a given size
    fn add(&mut self, size: u64) {
        self.count = self.count.saturating_add(1);
        self.total_size = self.total_size.saturating_add(size);
    }

    // Remove an item with a given size, if possible without underflowing
    fn remove(&mut self, size: u64) -> bool {
        if self.count > 1 && self.total_size > size {
            self.count -= 1;
            self.total_size -= size;
            true
        } else {
            false
        }
    }
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub list: StatefulList<DirEntryItem>,
    pub is_in_search_mode: bool,
    pub filter_input: Option<String>,
    pub deleting_size: Log,
    pub selected: ItemCounter,
    pub search_counter: u64,
    pub search_results: u64,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            list: StatefulList::default(),
            is_in_search_mode: false,
            filter_input: None,
            deleting_size: Log::default(),
            selected: ItemCounter::default(),
            search_counter: 0,
            search_results: 0,
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
        if self.list.mutate_selected(|item| {
            if item.can_toggle() {
                if item.is_on() {
                    item.set_is_on(false);
                    self.selected.add(item.size);
                } else {
                    self.selected.remove(item.size);
                    item.set_is_on(true);
                }
                true
            } else {
                false
            }
        }) {
            self.list.group_selection = None;
        }
    }

    pub fn start_search_entry(&mut self) {
        self.is_in_search_mode = true;
    }

    pub fn end_search_entry(&mut self) {
        self.is_in_search_mode = false;
    }

    pub fn set_on_and_next(&mut self) {
        self.list.mutate_selected(|item| {
            item.set_is_on(true);
            true
        });
        self.list.group_selection = None;
        self.list.next()
    }

    pub fn set_off_and_next(&mut self) {
        self.list.mutate_selected(|item| {
            item.set_is_on(false);
            true
        });
        self.list.group_selection = None;
        self.list.next()
    }

    pub fn next(&mut self) {
        self.list.next();
    }

    pub fn previous(&mut self) {
        self.list.previous();
    }

    pub fn toggle_group_selection(&mut self) {
        let group_selection = self
            .list
            .group_selection
            .as_ref()
            .map(|group_selection| match group_selection {
                GroupSelection::All => GroupSelection::None,
                GroupSelection::None => {
                    if self.list.visible_items().any(|item| item.is_on()) {
                        GroupSelection::None
                    } else {
                        GroupSelection::All
                    }
                }
            })
            .or(Some(GroupSelection::All));

        self.list.group_selection = group_selection;
    }

    pub fn handle_search(&mut self, d: DirSearch) {
        match d {
            DirSearch::Started => self.list.set_scanning(Some(ActionState::Pending)),
            DirSearch::Finished(counter, found) => {
                self.list.set_scanning(Some(ActionState::Done));
                self.search_counter = counter;
                self.search_results = found;
            }
            DirSearch::Found(e, size) => self.list.push(DirEntryItem::from_entry(e, size)),
            DirSearch::Progress(counter) => self.search_counter = counter,
        }
    }
    pub fn handle_delete(&mut self, d: DirDelete) {
        match d {
            DirDelete::Deleting(path) => {
                self.list.mutate_where(
                    |item| item.entry.path() == path,
                    |item| {
                        self.deleting_size.add(item.size);
                        item.delete_state = Some(ActionState::Pending)
                    },
                );
            }
            DirDelete::Deleted(path) => {
                self.list.mutate_where(
                    |item| item.entry.path() == path,
                    |item| {
                        self.deleting_size.finished(item.size);
                        item.delete_state = Some(ActionState::Done)
                    },
                );
            }
            DirDelete::Failed(path, error_message) => {
                self.list.mutate_where(
                    |item| item.entry.path() == path,
                    |item| {
                        self.deleting_size.failed(item.size);
                        item.delete_state = Some(ActionState::Failed(error_message));
                    },
                );
            }
        }
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
                self.is_in_search_mode = false;
            }
        }
    }
}
