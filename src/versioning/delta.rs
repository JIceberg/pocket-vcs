/// A delta is a difference in history for a recorded/staged
/// items during a commit.

use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub struct Delta<T> {
    prev: Option<T>,
    curr: Option<T>,
}

impl<T: Serialize> Serialize for Delta<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Delta", 2)?;
        s.serialize_field("previous", &self.prev)?;
        s.serialize_field("current", &self.curr)?;
        s.end()
    }
}

impl<T: Clone> Delta<T> {
    pub fn new() -> Self {
        Self {
            prev: None,
            curr: None,
        }
    }

    pub fn update(&mut self, item: T) -> Option<T> {
        let prev = match self.curr {
            Some(_) => self.prev.replace(self.curr.take().unwrap()),
            None => None
        };
        self.curr.replace(item);

        prev
    }

    pub fn get_current(&self) -> Option<T> {
        self.curr.clone()
    }
}