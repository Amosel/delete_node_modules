use crate::dir_entry_item::DirEntryItem;
use crate::event::{DirDelete, DirSearch, Event};
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
        s.send(Event::Delete(DirDelete::Deleting(item.entry.path().into())))
            .expect("Unable to send data through the channel.");
        let result: Result<(), std::io::Error> = std::fs::remove_dir_all(item.entry.path());
        match result {
            Ok(_) => {
                s.send(Event::Delete(DirDelete::Deleted(item.entry.path().into())))
                    .expect("Unable to send data through the channel.");
            }
            Err(e) => {
                s.send(Event::Delete(DirDelete::Failed(
                    item.entry.path().into(),
                    e.to_string(),
                )))
                .expect("Unable to send data through the channel.");
            }
        }
    });
}

#[cfg(target_os = "linux")]
fn get_directory_size(path: &str) -> std::io::Result<u64> {
    use std::process::Command;

    let output = Command::new("du")
        .arg("-s")
        .arg("--block-size=1")
        .arg(path)
        .output()?;

    let output_str = String::from_utf8(output.stdout)?;
    let size_str = output_str.split_whitespace().next().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse du output")
    })?;
    let size = size_str.parse::<u64>()?;
    Ok(size)
}

#[cfg(target_os = "macos")]
fn get_directory_size(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    // note this will only work on macos.
    let output = Command::new("du")
        .arg("-sk")
        .arg(path)
        .output()?;

    let output_str = String::from_utf8(output.stdout)?;
    let size_str = output_str.split_whitespace().next().ok_or("No output from du")?;
    let size_kb = size_str.parse::<u64>()?;
    let size_bytes = size_kb * 1024; // Convert from kilobytes to bytes
    Ok(size_bytes)
}

pub fn walk_node_modules(sender: Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        sender
            .send(Event::Search(DirSearch::Started))
            .expect("Unable to send data through the channel.");
        let path = Path::new("."); // Start from the current directory.
        let mut current: Option<PathBuf> = None;
        let mut counter: u64 = 0;
        let mut found: u64 = 0;
        for entry in WalkDir::new(path)
            .follow_links(false) // Do not follow symbolic links.
            .into_iter()
            .filter_map(Result::ok)
        {
            if counter % 100 == 0 {
                sender
                    .send(Event::Search(DirSearch::Progress(counter)))
                    .expect("Unable to send data through the channel.");
            }
            counter += 1;
            if entry.file_type().is_dir()
                && entry
                    .path()
                    .file_name()
                    .map_or(false, |name| name == "node_modules")
            {
                if let Some(ref previous) = current {
                    if entry.path().starts_with(previous) {
                        // Skip this entry because it's under a `node_modules` directory we've already processed.
                        continue;
                    }
                }
                // Update the current path and calculate size.
                current = Some(entry.path().to_path_buf());
                let size = entry
                    .path()
                    .to_str()
                    .map(|str| get_directory_size(str).unwrap_or(0))
                    .unwrap_or(0);
                // Send each valid directory entry through the channel.
                found += 1;
                sender
                    .send(Event::Search(DirSearch::Found(entry, size)))
                    .expect("Unable to send data through the channel.");
            }
        }
        sender
            .send(Event::Search(DirSearch::Finished(counter, found)))
            .expect("Unable to send finish event.");
    })
}
