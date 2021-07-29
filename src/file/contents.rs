use super::history::{History, Staged};
use super::header::Readable;

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

    pub fn print_history(&self) {
        println!("{}", self.history);
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

    fn current(&self) -> Option<Self::Item> {
        self.history.current()
    }

    fn revert(&mut self, commit: usize) -> Self::Item {
        let content = self.history.revert(commit);
        self.content = content;

        self.content.clone()
    }

    fn reset(&mut self) {
        self.history.reset();
        self.content = match self.current() {
            Some(c) => c.clone(),
            None => self.content.clone()
        };
    }

    fn stage(&mut self, item: Self::Item) {
        self.content = item.clone();
        self.history.stage(item);
    }
}