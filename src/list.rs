use crate::item::Toggle;
use tui::widgets::*;

pub trait SingleSelection {
    type Item; // Associated type to avoid forcing the same type for T in SingleSelection and StatefulList
    fn selected(&self) -> Option<&Self::Item>;
}

#[derive(Debug, Clone)]
pub struct StatefulList<T: Toggle> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T: Toggle> SingleSelection for StatefulList<T> {
    type Item = T;

    fn selected(&self) -> Option<&Self::Item> {
        // Here we handle the Option type, as self.state.selected is an Option<usize>
        // and return a reference to the item to avoid moving it out of the vector.
        self.state.selected().map(|index| &self.items[index])
    }
}

impl<T: Toggle> StatefulList<T> {
    pub fn new_empty() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if self.items.len() == 0 {
                    0
                } else if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn toggle(&mut self) {
        if let Some(item) = self.state.selected().map(|index| &mut self.items[index]) {
            item.toggle();
        }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
}
