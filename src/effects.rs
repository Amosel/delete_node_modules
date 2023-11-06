use crate::dir_entry_item::DirEntryItem;
use crate::event::{DirDeleteProcess, DirEntryProcess, Event};
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::Sender,
    thread::{self},
};
use walkdir::WalkDir;

pub fn delete_items(items: Vec<DirEntryItem>, sender: &Sender<Event>) {
    items.par_iter().for_each_with(sender.clone(), |s, item| {
        let _ = s.send(Event::Delete(DirDeleteProcess::Deleting((
            item.entry.path().into(),
            item.size,
        ))));
        let result: Result<(), std::io::Error> = std::fs::remove_dir_all(item.entry.path());
        match result {
            Ok(_) => {
                let _ = s.send(Event::Delete(DirDeleteProcess::Deleted((
                    item.entry.path().into(),
                    item.size,
                ))));
            }
            Err(e) => {
                let _ = s.send(Event::Delete(DirDeleteProcess::Failed(
                    (item.entry.path().into(), item.size),
                    e.to_string(),
                )));
            }
        }
    });
}

fn get_directory_size(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let output = Command::new("du")
        .arg("-sb") // Use '-sb' for size in bytes and summarize for the directory only
        .arg(path)
        .output()?;

    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "du command failed",
        )));
    }

    let output_str = std::str::from_utf8(&output.stdout)?;
    let size_str = output_str
        .split_whitespace()
        .next()
        .ok_or("No output from du")?;
    let size = size_str.parse::<u64>()?;
    Ok(size)
}

pub fn walk_node_modules(sender: Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = sender
            .send(Event::Entry(DirEntryProcess::Started))
            .expect("Unable to send data through the channel.");
        let path = Path::new("."); // Start from the current directory.
        let mut current: Option<PathBuf> = None;
        for entry in WalkDir::new(path)
            .follow_links(false) // Do not follow symbolic links.
            .into_iter()
            .filter_map(Result::ok)
        {
            if entry.file_type().is_dir() && entry.file_name() == "node_modules" {
                if let Some(ref previous) = current {
                    if entry.path().starts_with(previous) {
                        // Skip this entry because it's under a `node_modules` directory we've already processed.
                        continue;
                    }
                }
                // Update the current path and calculate size.
                current = Some(entry.path().to_path_buf());
                if let Ok(size) = get_directory_size(entry.path().to_str().unwrap()) {
                    // Send each valid directory entry through the channel.
                    let _ = sender
                        .send(Event::Entry(DirEntryProcess::Found(entry, size)))
                        .expect("Unable to send data through the channel.");
                }
            }
        }
        let _ = sender
            .send(Event::Entry(DirEntryProcess::Finished))
            .expect("Unable to send finish event.");
    })
}
