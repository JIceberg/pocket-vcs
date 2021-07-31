use std::collections::VecDeque;
use std::vec::Vec;

use super::delta::Delta;

#[derive(Clone)]
enum Commit<T> {
    Content(Delta<T>),
    Revert(Delta<T>, usize),
    Merge(T, T, Delta<T>),
}

impl<T: Clone> Commit<T> {
    fn get(&self) -> Option<T> {
        match &*self {
            Self::Content(x) => x.get_current(),
            Self::Revert(content, _) => content.get_current(),
            Self::Merge(_, _, result) => result.get_current(),
        }
    }

    fn get_delta(&self) -> Delta<T> {
        match &*self {
            Self::Content(x) => x.clone(),
            Self::Revert(content, _) => content.clone(),
            Self::Merge(_, _, result) => result.clone(),
        }
    }
}

pub struct History<T>(VecDeque<Commit<T>>, Vec<Commit<T>>);

impl<T: Clone> Clone for History<T> {
    fn clone(&self) -> Self {
        History(self.0.clone(), self.1.clone())
    }
}

impl<T: Clone> History<T> {
    pub fn start() -> Self {
        Self(VecDeque::new(), Vec::new())
    }

    pub fn get_current_delta(&self) -> Delta<T> {
        let delta = match self.0.back() {
            Some(back) => back.get_delta(),
            None => Delta::new()
        };

        delta
    }

    fn add(&mut self, item: T) {
        let mut delta = match self.0.back() {
            Some(back) => back.get_delta(),
            None => Delta::new()
        };
        delta.update(item);
        self.0.push_back(Commit::Content(delta));
    }

    fn add_unstaged(&mut self) {
        for item in &self.1 {
            self.0.push_back(item.clone());
        }
        self.clear_unstaged();
    }

    fn clear_unstaged(&mut self) {
        self.1.clear();
    }
}

pub trait Staged {
    type Item;

    // return the most recently staged item in history
    fn current(&self) -> Option<Self::Item>;

    // revert back to the specified commit
    fn revert(&mut self, commit: usize) -> Option<Self::Item>;

    // reset back to the most recently staged commit
    fn reset(&mut self);

    // push the current changes to history
    fn stage(&mut self, item: Self::Item);
}

impl<T: Clone> Staged for History<T> {
    type Item = T;

    fn current(&self) -> Option<Self::Item> {
        match self.0.back() {
            Some(commit) => commit.get(),
            None => None
        }
    }

    fn revert(&mut self, commit: usize) -> Option<Self::Item> {
        if self.0.is_empty() {
            panic!("No staged history.");
        }

        if !self.1.is_empty() {
            panic!("Please stage your changes before performing a revert.");
        }

        if commit.checked_sub(1).unwrap_or(self.0.len()) >= self.0.len() {
            panic!("Could not find commit {:?} in history", commit);
        }

        self.1.push(
            Commit::Revert(self.0[commit-1].get_delta(), commit)
        );

        self.0[commit-1].get()
    }

    fn reset(&mut self) {
        self.clear_unstaged();
    }

    fn stage(&mut self, item: Self::Item) {
        self.add_unstaged();
        self.add(item);
    }
}