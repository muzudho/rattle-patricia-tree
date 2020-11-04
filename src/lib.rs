pub mod sequence;
pub mod tree;

pub struct Sequence<T> {
    string: Vec<T>,
}
pub struct Tree<T> {
    root: Option<Box<Sequence<T>>>,
}
