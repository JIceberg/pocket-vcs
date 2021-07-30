use crate::versioning::history::{History, Staged};
use std::{fmt, str};

pub trait Readable {
    type Item;
    fn read(&self) -> Self::Item;
}

#[derive(Clone)]
struct Byteset<'a>(Option<&'a [u8]>);

impl<'a> fmt::Display for Byteset<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(str::from_utf8(self.0.unwrap_or(&[])).unwrap());
        write!(f, "{}", buf)
    }
}

impl<'a> Byteset<'a> {
    fn get(&self) -> &'a [u8] {
        self.0.unwrap_or(&[]).clone()
    }

    fn set(&mut self, bytes: &'a [u8]) {
        self.0.replace(bytes);
    }
}

pub struct Line<'a> {
    content: Byteset<'a>,
    history: Box<History<Byteset<'a>>>,
}

impl<'a> Clone for Line<'a> {
    fn clone(&self) -> Self {
        Line {
            content: self.content.clone(),
            history: self.history.clone()
        }
    }
}

impl<'a> Line<'a> {
    pub(crate) fn new() -> Self {
        Self {
            content: Byteset(None),
            history: Box::new(History::start())
        }
    }

    pub fn print_history(&self) {
        println!("{}", self.history);
    }
}

impl<'a> Readable for Line<'a> {
    type Item = &'a [u8];
    
    fn read(&self) -> Self::Item {
        self.content.get()
    }
}

impl<'a> Staged for Line<'a> {
    type Item = &'a [u8];

    fn current(&self) -> Option<Self::Item> {
        match self.history.current() {
            Some(curr) => Some(curr.get()),
            None => None
        }
    }

    fn revert(&mut self, commit: usize) -> Self::Item {
        let content = self.history.revert(commit);
        self.content.set(content.get());

        self.content.get()
    }

    fn reset(&mut self) {
        self.history.reset();
        match self.current() {
            Some(curr) => self.content.set(curr),
            None => {}
        }
    }

    fn stage(&mut self, item: Self::Item) {
        self.content.set(item);
        self.history.stage(self.content.clone());
    }
}