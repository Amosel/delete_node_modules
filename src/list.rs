use crate::dir_entry_item::{DirEntryItem, DirEntryItemList, Toggle};
use tui::widgets::*;

pub trait SingleSelection {
    type Item; // Associated type to avoid forcing the same type for T in SingleSelection and StatefulList
    fn selected(&self) -> Option<&Self::Item>;
}

pub trait Filterable<'a, T> {
    fn apply_filter<F>(&mut self, filter: F)
    where
        F: Fn(&T) -> bool;
    fn clear_filter(&mut self);

    fn visible_items<'b>(&'b self) -> Box<dyn Iterator<Item = &'b T> + 'b>;
}

pub trait AsyncContent<T> {
    fn scanning(&self) -> bool;
    fn set_scanning(&mut self);
    fn set_done_scanning(&mut self);
    fn push(&mut self, item: T);
}

#[derive(Debug, Clone)]
pub struct StatefulList<T: Toggle> {
    items: Vec<T>,
    pub state: ListState,
    scanning: bool,
    filtered: Option<Vec<usize>>,
}

impl<'a, T: Toggle> AsyncContent<T> for StatefulList<T> {
    fn scanning(&self) -> bool {
        self.scanning
    }
    fn set_done_scanning(&mut self) {
        self.scanning = false;
    }
    fn set_scanning(&mut self) {
        self.scanning = true;
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
}

impl<'a, T: Toggle> SingleSelection for StatefulList<T> {
    type Item = T;
    fn selected(&self) -> Option<&Self::Item> {
        // Here we handle the Option type, as self.state.selected is an Option<usize>
        // and return a reference to the item to avoid moving it out of the vector.
        if let Some(filtered) = &self.filtered {
            self.state.selected().and_then(|index| {
                filtered
                    .get(index)
                    .copied()
                    .and_then(|index| self.items.get(index))
            })
        } else {
            self.state
                .selected()
                .and_then(|index| self.items.get(index))
        }
    }
}

impl<'a, T: Toggle> Filterable<'a, T> for StatefulList<T> {
    fn apply_filter<F>(&mut self, filter: F)
    where
        F: Fn(&T) -> bool,
    {
        self.filtered = Some(
            self.items
                .iter()
                .enumerate()
                .filter_map(|(index, item)| {
                    if filter(item) {
                        Some(index)
                    } else {
                        if self.state.selected().map(|i| i == index).unwrap_or(false) {
                            self.state.select(None)
                        }
                        None
                    }
                })
                .collect(),
        );
    }

    fn clear_filter(&mut self) {
        self.filtered = None;
    }

    fn visible_items<'b>(&'b self) -> Box<dyn Iterator<Item = &'b T> + 'b> {
        if let Some(filtered) = &self.filtered {
            Box::new(
                filtered
                    .iter()
                    .filter_map(move |&index| self.items.get(index)),
            )
        } else {
            Box::new(self.items.iter())
        }
    }
}

impl<'a, T: Toggle> StatefulList<T> {
    pub fn new_empty() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
            filtered: None,
            scanning: false,
        }
    }
    pub fn new(items: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            items,
            filtered: None,
            scanning: false,
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected().and_then(|index| {
            if let Some(filterd_indices) = &self.filtered {
                if filterd_indices.contains(&index) {
                    Some(index)
                } else {
                    None
                }
            } else {
                Some(index)
            }
        })
    }
    pub fn selected(&self) -> Option<&T> {
        self.selected_index().map(|index| &self.items[index])
    }

    fn selected_mut(&mut self) -> Option<&mut T> {
        if let Some(index) = self.selected_index() {
            self.items.get_mut(index)
        } else {
            None
        }
    }

    pub fn set_on_and_next(&mut self) {
        self.selected_mut().map(|item| item.set_is_on(true));
        self.next()
    }

    pub fn set_off_and_next(&mut self) {
        self.selected_mut().map(|item| item.set_is_on(false));
        self.next()
    }

    pub fn next(&mut self) {
        let next_index = if let Some(filtered) = &self.filtered {
            // Handle the case when a filter is applied
            self.state
                .selected()
                .and_then(|selected| {
                    filtered
                        .iter()
                        .position(|&value| value == selected)
                        .and_then(|idx| Some((idx + 1) % filtered.len()))
                        .and_then(|filtered_idx| filtered.get(filtered_idx))
                        .copied()
                })
                .or_else(|| filtered.first().copied())
        } else {
            // Handle the case when no filter is applied
            self.state
                .selected()
                .and_then(|selected| Some((selected + 1) % self.items.len()))
                .or_else(|| if self.items.len() == 0 { None } else { Some(0) })
        };

        self.state.select(next_index);
    }

    pub fn previous(&mut self) {
        let next_index = if let Some(filtered) = &self.filtered {
            // Handle the case when a filter is applied
            self.state
                .selected()
                .and_then(|selected| {
                    filtered
                        .iter()
                        .position(|&value| value == selected)
                        .and_then(|idx| Some((idx + filtered.len() - 1) % filtered.len()))
                        .and_then(|filtered_idx| filtered.get(filtered_idx))
                        .copied()
                })
                .or_else(|| filtered.first().copied())
        } else {
            // Handle the case when no filter is applied
            self.state
                .selected()
                .and_then(|selected| Some((selected + self.items.len() - 1) % self.items.len()))
                .or_else(|| {
                    if self.items.len() == 0 {
                        None
                    } else {
                        Some(self.items.len() - 1)
                    }
                })
        };
        self.state.select(next_index);
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn toggle_selected_item(&mut self) {
        if let Some(item) = self.state.selected().map(|index| &mut self.items[index]) {
            item.toggle();
        }
    }
}

impl DirEntryItemList for StatefulList<DirEntryItem> {
    fn delete(&mut self, e: walkdir::DirEntry) {
        if let Some(index) = self
            .items
            .iter()
            .position(|item| item.entry.path() != e.path())
        {
            self.items.remove(index);
            if let Some(filtered) = self.filtered.as_mut() {
                filtered.retain(|&i| i != index);
            }
        }
    }

    fn set_deleting(&mut self, e: walkdir::DirEntry) {
        if let Some(item) = self
            .items
            .iter_mut()
            .find(|item| item.entry.path() == e.path())
        {
            item.deleting = true;
        }
    }
    fn set_failed(&mut self, e: walkdir::DirEntry, error_message: String) {
        if let Some(item) = self
            .items
            .iter_mut()
            .find(|item| item.entry.path() == e.path())
        {
            item.error_message = Some(error_message);
        }
    }
}
