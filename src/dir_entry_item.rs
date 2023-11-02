use walkdir::DirEntry;

pub trait Toggle {
    // Toggle the selection state.
    fn toggle(&mut self);

    // Set the selection state explicitly.
    fn set_is_on(&mut self, is_on: bool);

    // Check whether the item is on.
    fn is_on(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct DirEntryItem {
    pub entry: DirEntry,
    pub is_on: bool,
    pub size: u64,
    pub deleting: bool,
    pub error_message: Option<String>,
}

impl DirEntryItem {
    pub fn from_entry(entry: DirEntry, size: u64) -> DirEntryItem {
        DirEntryItem {
            entry,
            size,
            is_on: false,
            deleting: false,
            error_message: None,
        }
    }
}

impl Toggle for DirEntryItem {
    fn toggle(&mut self) {
        if self.deleting || self.error_message.is_some() {
            return;
        }
        self.is_on = !self.is_on;
    }

    fn set_is_on(&mut self, is_on: bool) {
        if self.deleting || self.error_message.is_some() {
            return;
        }
        self.is_on = is_on;
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
}

pub trait DirEntryItemList {
    fn set_deleting(&mut self, e: DirEntry);
    fn set_failed(&mut self, e: DirEntry, error_message: String);
    fn delete(&mut self, e: DirEntry);
}
