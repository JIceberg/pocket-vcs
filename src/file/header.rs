pub trait Readable {
    type Item;
    fn read(&self) -> Self::Item;
}