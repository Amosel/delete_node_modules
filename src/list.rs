use crate::{actions::ActionState, app::GroupSelection};
use tui::widgets::*;

pub trait Toggle {
    // Toggle the selection state.
    fn toggle(&mut self);

    // Set the selection state explicitly.
    fn set_is_on(&mut self, is_on: bool);

    // Check whether the item is on.
    fn is_on(&self) -> bool;
}

pub trait SingleSelection {
    type Item; // Associated type to avoid forcing the same type for T in SingleSelection and StatefulList
    fn selected(&self) -> Option<&Self::Item>;
}

pub trait GroupSelectable {
    fn toggle_group_selection(&mut self);
}

pub trait Deletable {
    fn can_delete(&self) -> bool;
}

pub trait Filterable<'a, T> {
    fn apply_filter<F>(&mut self, filter: F)
    where
        F: Fn(&T) -> bool;

    fn clear_filter(&mut self);

    fn visible_items<'b>(&'b self) -> Box<dyn Iterator<Item = &'b T> + 'b>;
}

#[derive(Debug, Clone)]
pub struct StatefulList<T: Toggle> {
    items: Vec<T>,
    pub state: ListState,
    scanning: Option<ActionState>,
    filtered_indices: Option<Vec<usize>>,
    pub group_selection: Option<GroupSelection>,
}

impl<T: Toggle> SingleSelection for StatefulList<T> {
    type Item = T;
    fn selected(&self) -> Option<&Self::Item> {
        // Here we handle the Option type, as self.state.selected is an Option<usize>
        // and return a reference to the item to avoid moving it out of the vector.
        if let Some(filtered) = &self.filtered_indices {
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
        self.filtered_indices = Some(
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
        self.filtered_indices = None;
    }

    fn visible_items<'b>(&'b self) -> Box<dyn Iterator<Item = &'b T> + 'b> {
        self.filtered_indices
            .as_ref()
            .map_or(Box::new(self.items.iter()), |filtered_indices| {
                Box::new(
                    filtered_indices
                        .iter()
                        .filter_map(move |&idx| self.items.get(idx)),
                )
            })
    }
}

impl<T: Toggle> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
            filtered_indices: None,
            scanning: None,
            group_selection: None,
        }
    }
}

impl<T: Toggle> StatefulList<T> {

    pub fn has_visible_items(&self) -> bool {
        self.visible_items().next().is_some()
    }
    pub fn is_scanning(&self) -> bool {
        matches!(self.scanning, Some(ActionState::Pending))
    }
    pub fn done_scanning(&self) -> bool {
        matches!(self.scanning, Some(ActionState::Done))
    }

    pub fn set_scanning(&mut self, state: Option<ActionState>) {
        self.scanning = state;
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn next(&mut self) {
        let next_index = if let Some(filtered) = &self.filtered_indices {
            // Handle the case when a filter is applied
            self.state
                .selected()
                .and_then(|selected| {
                    filtered
                        .iter()
                        .position(|&value| value == selected)
                        .map(|idx| (idx + 1) % filtered.len())
                        .and_then(|filtered_idx| filtered.get(filtered_idx))
                        .copied()
                })
                .or_else(|| filtered.first().copied())
        } else {
            // Handle the case when no filter is applied
            self.state
                .selected()
                .map(|selected| (selected + 1) % self.items.len())
                .or({
                    if self.items.is_empty() {
                        None
                    } else {
                        Some(0)
                    }
                })
        };

        self.state.select(next_index);
    }

    pub fn previous(&mut self) {
        let next_index = if let Some(filtered) = &self.filtered_indices {
            // Handle the case when a filter is applied
            self.state
                .selected()
                .and_then(|selected| {
                    filtered
                        .iter()
                        .position(|&value| value == selected)
                        .map(|idx| (idx + filtered.len() - 1) % filtered.len())
                        .and_then(|filtered_idx| filtered.get(filtered_idx))
                        .copied()
                })
                .or_else(|| filtered.first().copied())
        } else {
            // Handle the case when no filter is applied
            self.state
                .selected()
                .map(|selected| (selected + self.items.len() - 1) % self.items.len())
                .or({
                    if self.items.is_empty() {
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

    pub fn mutate_selected<F>(&mut self, mutator: F) -> bool
    where
        F: FnOnce(&mut T) -> bool,
    {
        if let Some(item) = self
            .state
            .selected()
            .and_then(|idx| self.items.get_mut(idx))
        {
            mutator(item)
        } else {
            false
        }
    }

    pub fn mutate_where<F, P>(&mut self, predicate: P, mutator: F) -> bool
    where
        F: FnOnce(&mut T),
        P: Fn(&T) -> bool,
    {
        if let Some(item) = self.items.iter_mut().find(|item| predicate(item)) {
            mutator(item);
            true
        } else {
            false
        }
    }
}

impl<T: Toggle + Deletable> StatefulList<T> {
    pub fn items_to_delete<'b>(&'b self) -> Box<dyn Iterator<Item = &'b T> + 'b> {
        if let Some(group_selection) = self.group_selection.as_ref() {
            match group_selection {
                GroupSelection::All => return self.visible_items(),
                GroupSelection::None => return Box::new(std::iter::empty()),
            }
        }
        Box::new(self.visible_items().filter(|item| item.can_delete()))
    }
}
