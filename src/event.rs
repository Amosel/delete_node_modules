use crossterm::event::{KeyEvent, MouseEvent};
use std::path::PathBuf;
use walkdir::DirEntry;

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
