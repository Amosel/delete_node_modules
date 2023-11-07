use crossterm::event::{KeyEvent, MouseEvent};
use std::path::PathBuf;
use walkdir::DirEntry;

#[derive(Clone, Debug)]
pub enum DirSearch {
    Started,
    Finished(u64,u64),
    Found(DirEntry, u64),
    Progress(u64)
}

#[derive(Clone, Debug)]
pub enum DirDelete {
    Deleting(PathBuf),
    Deleted(PathBuf),
    Failed(PathBuf, String),
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

    Delete(DirDelete),
    Search(DirSearch),
}
