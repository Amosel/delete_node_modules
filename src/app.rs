pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
use crate::dir_entry_item::DirEntryItem;
use crate::event::{DirDeleteProcess, DirEntryProcess, Event};
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
    pub deleting: usize,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            list: StatefulList::new_empty(),
            loading: true,
            group_selection: GroupSelection::Selected,
            deleting: 0,
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

    pub fn on_entry(&mut self, d: DirEntryProcess) {
        match d {
            DirEntryProcess::Started => self.loading = true,
            DirEntryProcess::Finished => self.loading = false,
            DirEntryProcess::Found(e, size) => self.list.push(DirEntryItem::from_entry(e, size)),
        }
    }
    pub fn on_delete(&mut self, d: DirDeleteProcess) {
        match d {
            DirDeleteProcess::Started(d) => self.deleting = d,
            DirDeleteProcess::Finished(_) => self.deleting = 0,
            DirDeleteProcess::Deleting(e) => {
                if let Some(item) = self
                    .list
                    .items
                    .iter_mut()
                    .find(|item| item.entry.path() == e.path())
                {
                    item.deleting = true;
                }
            }
            DirDeleteProcess::Deleted(e) => {
                self.list.items.retain(|item| item.entry.path() != e.path());
            }
            DirDeleteProcess::Failed(_) => todo!(),
        }
    }

    pub fn process_deletes(&mut self, sender: std::sync::mpsc::Sender<Event>) {
        let items = match self.group_selection {
            GroupSelection::Selected => self.list.items.iter().cloned().collect(),
            GroupSelection::Deselected => {
                vec![]
            }
            GroupSelection::None => self
                .list
                .items
                .iter()
                .filter(|item| item.is_on)
                .cloned()
                .collect(),
        };
        if items.is_empty() {
            return;
        }

        let _ = sender.send(Event::Delete(DirDeleteProcess::Started(items.len())));

        for item in items {
            let sender = sender.clone();
            std::thread::spawn(move || {
                let _ = sender.send(Event::Delete(DirDeleteProcess::Deleting(
                    item.entry.clone(),
                )));
                let result = std::fs::remove_dir_all(item.entry.path());
                match result {
                    Ok(_) => {
                        let _ = sender.send(Event::Delete(DirDeleteProcess::Deleted(item.entry)));
                    }
                    Err(_) => {
                        let _ = sender.send(Event::Delete(DirDeleteProcess::Failed(item.entry)));
                    }
                }
            });
        }
    }
}
