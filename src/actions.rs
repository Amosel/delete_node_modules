#[derive(Debug, Clone)]
pub enum ActionState {
    Pending,
    Done,
    Failed(String),
}

impl ActionState {
    pub fn loaded(&self) -> bool {
        matches!(self, ActionState::Done)
    }
}

#[derive(Debug, PartialEq)]
pub struct Actions<T: PartialEq> {
    pub history: Option<Vec<T>>,
    pub queued: Option<Vec<T>>,
    pub current: Option<Vec<T>>,
    pub failed: Option<Vec<(T, String)>>,
}

impl<T: PartialEq> Default for Actions<T> {
    fn default() -> Self {
        Self {
            queued: None,
            history: None,
            current: None,
            failed: None,
        }
    }
}
impl<T: PartialEq> Actions<T> {
    pub fn add_to_queue(&mut self, item: T) {
        self.queued.get_or_insert_with(Vec::new).push(item);
    }
    // A helper function to reduce repetition
    fn move_item_to_vec(item: T, from: &mut Option<Vec<T>>, to: &mut Option<Vec<T>>) {
        if let Some(vec) = from {
            vec.retain(|i| *i != item);
        }
        to.get_or_insert_with(Vec::new).push(item);
    }

    pub fn add_to_current(&mut self, item: T) {
        Self::move_item_to_vec(item, &mut self.queued, &mut self.current);
    }

    pub fn add_to_history(&mut self, item: T) {
        Self::move_item_to_vec(item, &mut self.current, &mut self.history);
    }

    pub fn add_to_failed(&mut self, item: T, error_message: String) {
        if let Some(vec) = self.current.as_mut() {
            vec.retain(|i| *i == item);
        }
        self.failed
            .get_or_insert_with(Vec::new)
            .push((item, error_message))
    }
}