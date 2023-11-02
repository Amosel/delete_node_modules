use crate::{app::AppResult, dir_entry_item::DirEntryItem};
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use rayon::prelude::*;
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self},
    time::{Duration, Instant},
};

use walkdir::{DirEntry, WalkDir};

#[derive(Clone, Debug)]
pub enum DirEntryProcess {
    Started,
    Finished,
    Found(DirEntry, u64),
}

#[derive(Clone, Debug)]
pub enum DirDeleteProcess {
    Deleting((PathBuf, u64)),
    Deleted((PathBuf, u64)),
    Failed((PathBuf, u64), String),
}

/// Terminal events.
#[derive(Clone, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),

    Delete(DirDeleteProcess),
    Entry(DirEntryProcess),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    pub sender: Sender<Event>,
    /// Event receiver channel.
    pub receiver: Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

pub fn delete(items: Vec<DirEntryItem>, sender: &Sender<Event>) {
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

fn calculate_dir_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let mut size = 0;

    for entry in WalkDir::new(path)
        .min_depth(1) // skip the root node_modules directory itself
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            size += metadata.len();
        }
    }

    Ok(size)
}

fn lifecycle(sender: Sender<Event>, tick_rate: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(tick_rate);

            if event::poll(timeout).expect("no events available") {
                match event::read().expect("unable to read event") {
                    CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                    CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                    CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                    _ => unimplemented!(),
                }
                .expect("failed to send terminal event")
            }

            if last_tick.elapsed() >= tick_rate {
                sender.send(Event::Tick).expect("failed to send tick event");
                last_tick = Instant::now();
            }
        }
    })
}

fn walk(sender: Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        sender
            .send(Event::Entry(DirEntryProcess::Started))
            .expect("Unable to send data through the channel.");
        let path = Path::new("."); // Start from the current directory.
        let mut current: Option<PathBuf> = None;
        for entry in WalkDir::new(path)
            .follow_links(false) // Follow symbolic links.
            .into_iter()
            .filter_map(Result::ok)
        // Filter out potential errors during iteration.
        {
            if entry.file_type().is_dir() && entry.file_name().to_string_lossy() == "node_modules" {
                if let Some(ref previous) = current {
                    if !entry.path().starts_with(previous) {
                        current = Some(entry.path().into());
                        let size: u64 = calculate_dir_size(entry.path()).unwrap();
                        sender
                            .send(Event::Entry(DirEntryProcess::Found(entry, size)))
                            .expect("Unable to send data through the channel.");
                    }
                } else {
                    current = Some(entry.path().into());
                    let size: u64 = calculate_dir_size(entry.path()).unwrap();
                    sender
                        .send(Event::Entry(DirEntryProcess::Found(entry, size)))
                        .expect("Unable to send data through the channel.");
                }
            }
            // Send each valid directory entry through the channel.
        }
        sender
            .send(Event::Entry(DirEntryProcess::Finished))
            .expect("Unable to send finish event.");
    })
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (ui_sender, receiver) = channel();
        let handler = {
            lifecycle(ui_sender.clone(), tick_rate);
            walk(ui_sender.clone())
        };

        Self {
            sender: ui_sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> AppResult<Event> {
        Ok(self.receiver.recv()?)
    }

    pub fn get_sender(&self) -> Sender<Event> {
        self.sender.clone()
    }
}
