pub mod sequence;
pub mod tree;

#[derive(Clone)]
pub struct Sequence<T> {
    sequence: Vec<T>,
}

#[derive(Clone)]
pub struct Tree<T> {
    root: Option<Sequence<T>>,
}
