use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::fs;

pub trait Toggle {
    // Toggle the selection state.
    fn toggle(&mut self);

    // Set the selection state explicitly.
    fn set_is_on(&mut self, is_on: bool);

    // Check whether the item is on.
    fn is_on(&self) -> bool;
}

#[derive(Debug)]
pub struct Item {
    pub path: PathBuf,
    pub size_mb: f64,
    pub is_on: bool,
}

fn calculate_directory_size(path: &Path) -> Result<f64, std::io::Error> {
    let mut size = 0;
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let metadata = fs::metadata(entry.path())?;
        size += metadata.len();
    }
    Ok((size as f64) / 1_048_576 as f64) // convert bytes to MB
}

impl Item {
    pub fn from_path(path: PathBuf) -> Result<Item, std::io::Error> {
        let size_mb = calculate_directory_size(&path)?;
        Ok(Item {
            path,
            size_mb,
            is_on: false,
        })
    }
    pub fn title(&self) -> String {
        self.path.to_str().unwrap().to_string() + " - " + &self.size_mb.to_string() + " MB"
    }
}

impl Toggle for Item {
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
