use std::collections::LinkedList;

#[derive(Clone)]
pub struct History<T>(LinkedList<T>);

impl<T> History<T> {
    pub fn start() -> Self {
        Self(LinkedList::new())
    }

    fn add(&mut self, item: T) {
        self.0.push_back(item);
    }
}

pub trait Staged {
    type Item;

    fn current(&self) -> Option<&Self::Item>;

    fn revert(&mut self, steps: usize) -> LinkedList<Self::Item>;

    fn stage(&mut self, item: Self::Item);
}

impl<T: Eq> Staged for History<T> {
    type Item = T;

    fn current(&self) -> Option<&Self::Item> {
        self.0.back()
    }

    fn revert(&mut self, steps: usize) -> LinkedList<Self::Item> {
        use std::cmp::{min, max};
        self.0.split_off(max(1, self.0.len() - min(steps, self.0.len())))
    }

    fn stage(&mut self, item: Self::Item) {
        match self.current() {
            Some(curr) => {
                if *curr != item {
                    self.add(item);
                }
            },
            None => self.add(item)
        }
    }
}