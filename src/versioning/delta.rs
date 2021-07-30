/// A delta is a difference between two recorded/staged
/// items.

pub struct Delta<T>(Option<T>, Option<T>);

impl<T> Delta<T> {
    pub fn new() -> Self {
        Self(None, None)
    }

    pub fn update(&mut self, item: T) -> Option<T> {
        let prev = match self.1 {
            Some(_) => self.0.replace(self.1.take().unwrap()),
            None => None
        };
        self.1.replace(item);

        prev
    }
}