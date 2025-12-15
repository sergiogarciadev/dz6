use std::collections::HashSet;

use tui_input::Input;

use crate::config::CMD_INPUT_HIST_SIZE;

#[derive(Default)]
pub struct InputHistory {
    pub input: Input,
    pub history: Vec<String>,
    pub history_index: Option<usize>,
    history_set: HashSet<String>,
}

impl InputHistory {
    pub fn push(&mut self, entry: String) {
        if entry.trim().is_empty() {
            return;
        }
        if !self.history_set.insert(entry.clone()) {
            return;
        }
        self.history.push(entry);
        // limit size
        if self.history.len() > CMD_INPUT_HIST_SIZE {
            let old = self.history.remove(0);
            self.history_set.remove(&old);
        }
        self.history_index = None;
    }

    pub fn up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let len = self.history.len();
        let new_index = match self.history_index {
            None => len - 1,
            Some(0) => 0,
            Some(i) => i - 1,
        };
        self.history_index = Some(new_index);
        self.input = Input::new(self.history[new_index].clone());
    }

    pub fn down(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let len = self.history.len();
        match self.history_index {
            None => {}
            Some(i) if i >= len - 1 => {
                self.history_index = None;
                self.input = Input::default();
            }
            Some(i) => {
                self.history_index = Some(i + 1);
                self.input = Input::new(self.history[i + 1].clone());
            }
        }
    }
}
