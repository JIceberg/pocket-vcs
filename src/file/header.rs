/// A "header" is meant to serve as saved
/// metadata for the version control scheme.
/// Since this program can only run once and isn't constantly
/// running repeatedly, there has to be somewhere that the
/// data is stored so that it can be re-accessed and
/// restored into the program.
/// 
/// The way that this is managed is by storing diffs, or the
/// difference between commits in history. Because we only care about
/// the differences, we can store each difference in a custom
/// version file type that has a structure readable by this
/// program.

pub trait Readable {
    type Item;
    fn read(&self) -> Self::Item;
}