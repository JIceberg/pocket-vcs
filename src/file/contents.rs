use super::history::{History, Staged};
use std::collections::LinkedList;

pub trait Readable {
    type Item;
    fn read(&self) -> Self::Item;
}

pub struct Line {
    content: String,
    history: Box<History<String>>,
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line {
            content: self.content.clone(),
            history: self.history.clone()
        }
    }
}

impl Line {
    pub(crate) fn new() -> Self {
        Self {
            content: String::new(),
            history: Box::new(History::start())
        }
    }
}

impl Readable for Line {
    type Item = String;
    
    fn read(&self) -> Self::Item {
        self.content.clone()
    }
}

impl Staged for Line {
    type Item = String;

    fn current(&self) -> Option<&Self::Item> {
        self.history.current()
    }

    fn revert(&mut self, steps: usize) -> LinkedList<Self::Item> {
        let saved = self.history.revert(steps);
        self.content = match self.current() {
            Some(c) => c.clone(),
            None => self.content.clone()
        };
        saved
    }

    fn stage(&mut self, item: Self::Item) {
        self.content = item.clone();
        self.history.stage(item);
    }
}