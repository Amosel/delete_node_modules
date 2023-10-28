use anyhow::{Context, Result};
use std::fs;
use walkdir::DirEntry;

pub trait Toggle {
    // Toggle the selection state.
    fn toggle(&mut self);

    // Set the selection state explicitly.
    fn set_is_on(&mut self, is_on: bool);

    // Check whether the item is on.
    fn is_on(&self) -> bool;
}

#[derive(Debug)]
pub struct DirEntryItem {
    pub entry: DirEntry,
    pub is_on: bool,
}

impl DirEntryItem {
    pub fn from_entry(entry: DirEntry) -> Result<DirEntryItem> {
        Ok(DirEntryItem {
            entry,
            is_on: false,
        })
    }
    pub fn title(&self) -> String {
        self.entry.path().to_str().unwrap().to_string()
            + " - "
            + &self.size_mb().unwrap().to_string()
            + " MB"
    }

    fn size_mb(&self) -> Result<f64, std::io::Error> {
        let metadata = fs::metadata(self.entry.path())
            .context("get the entry's metadata")
            .unwrap();
        let size = metadata.len();
        Ok((size as f64) / 1_048_576 as f64) // convert bytes to MB
    }
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
