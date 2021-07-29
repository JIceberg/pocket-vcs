use std::collections::VecDeque;
use std::vec::Vec;
use std::fmt;

enum Commit<T> {
    Content(T),
    Revert(T, usize),
    Merge(T, T),
}

impl<T: Clone> Commit<T> {
    fn get(&self) -> T {
        match &*self {
            Self::Content(x) => x.clone(),
            Self::Revert(content, _) => content.clone(),
            Self::Merge(_, modified) => modified.clone(),
        }
    }
}

impl<T: Clone> Clone for Commit<T> {
    fn clone(&self) -> Self {
        match &*self {
            Self::Content(x) => Self::Content(x.clone()),
            Self::Revert(content, idx) => Self::Revert(content.clone(), *idx),
            Self::Merge(previous, modified) => Self::Merge(previous.clone(), modified.clone())
        }
    }
}

impl<T: fmt::Display> fmt::Display for Commit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res = match &*self {
            Self::Content(content) => format!("{}", content),
            Self::Revert(content, commit) => format!("REVERT({}) => {}", commit, content),
            Self::Merge(previous, modified) => format!("MERGE {} => {}", previous, modified),
        };

        write!(f, "{}", res)
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

    fn add(&mut self, item: T) {
        self.0.push_back(Commit::Content(item));
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

impl<T: fmt::Display> fmt::Display for History<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x: String = match self.0.front() {
            Some(commit) => format!("1:\t{}\n", commit),
            None => String::from("No history")
        };
        for i in 1..self.0.len() {
            x.push_str(format!("{}:\t{}\n", i+1, self.0[i]).as_str());
        }
        writeln!(f, "{}", x)
    }
}

pub trait Staged {
    type Item;

    // return the most recently staged item in history
    fn current(&self) -> Option<Self::Item>;

    // revert back to the specified commit
    fn revert(&mut self, commit: usize) -> Self::Item;

    // reset back to the most recently staged commit
    fn reset(&mut self);

    // push the current changes to history
    fn stage(&mut self, item: Self::Item);
}

impl<T: Eq + Clone> Staged for History<T> {
    type Item = T;

    fn current(&self) -> Option<Self::Item> {
        match self.0.back() {
            Some(commit) => Some(commit.get()),
            None => None
        }
    }

    fn revert(&mut self, commit: usize) -> Self::Item {
        if self.0.is_empty() {
            panic!("No staged history.");
        }

        if !self.1.is_empty() {
            panic!("Please stage your changes before performing a revert.");
        }

        if commit > self.0.len() {
            panic!("Could not find commit {:?} in history", commit);
        }

        let ret = self.0[commit-1].get();
        self.1.push(
            Commit::Revert(ret.clone(), commit)
        );

        ret
    }

    fn reset(&mut self) {
        self.clear_unstaged();
    }

    fn stage(&mut self, item: Self::Item) {
        self.add_unstaged();
        match self.current() {
            Some(curr) => {
                if curr != item {
                    self.add(item);
                }
            },
            None => self.add(item)
        }
    }
}