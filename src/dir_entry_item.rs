use anyhow::Result;
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
}

impl DirEntryItem {
    pub fn from_entry(entry: DirEntry, size: u64) -> Result<DirEntryItem> {
        Ok(DirEntryItem {
            entry,
            size,
            is_on: false,
        })
    }
    // pub fn size(&self) -> Result<u64, std::io::Error> {
    //     let metadata = fs::metadata(self.entry.path())
    //         .context("get the entry's metadata")
    //         .unwrap();
    //     Ok(metadata.len())
    // }
}

impl Toggle for DirEntryItem {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }

    fn set_is_on(&mut self, is_on: bool) {
        self.is_on = is_on;
    }

    fn is_on(&self) -> bool {
        self.is_on
    }
}
