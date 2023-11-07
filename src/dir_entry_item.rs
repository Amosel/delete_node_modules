use walkdir::DirEntry;
use crate::actions::ActionState;
use crate::list::{Toggle, Deletable};

#[derive(Debug, Clone)]
pub struct DirEntryItem {
    pub entry: DirEntry,
    pub size: u64,
    pub delete_state: Option<ActionState>,
    is_on: bool,
}

impl DirEntryItem {
    pub fn from_entry(entry: DirEntry, size: u64) -> DirEntryItem {
        DirEntryItem {
            entry,
            size,
            is_on: false,
            delete_state: None,
        }
    }
    pub fn can_toggle(&self) -> bool {
        self.delete_state.is_none()
    }

    pub fn is_deleting(&self) -> bool {
        matches!(self.delete_state, Some(ActionState::Pending))
    }
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

impl Toggle for DirEntryItem {
    fn toggle(&mut self) {
        if self.can_toggle() {
            self.is_on = !self.is_on;
        }
    }

    fn set_is_on(&mut self, is_on: bool) {
        if self.can_toggle() {
            self.is_on = is_on;
        }
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
}

impl Deletable for DirEntryItem {
    fn can_delete(&self) -> bool {
        self.is_on() && self.delete_state.is_none()
    }
}