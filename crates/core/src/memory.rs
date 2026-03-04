use std::collections::VecDeque;

use crate::message::Message;

#[derive(Debug, Clone)]
pub struct MemoryStore {
    short_term: VecDeque<Message>,
    max_short_term: usize,
}

impl MemoryStore {
    pub fn new(max_short_term: usize) -> Self {
        Self {
            short_term: VecDeque::with_capacity(max_short_term),
            max_short_term,
        }
    }

    pub fn push(&mut self, message: Message) {
        self.short_term.push_back(message);
        while self.short_term.len() > self.max_short_term {
            let _ = self.short_term.pop_front();
        }
    }

    pub fn recent(&self) -> impl Iterator<Item = &Message> {
        self.short_term.iter()
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new(128)
    }
}
