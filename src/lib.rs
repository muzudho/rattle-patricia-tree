pub mod sequence;
pub mod tree;

#[derive(Clone)]
pub struct Sequence<T> {
    sequence: Vec<T>,
    cursor: usize,
}

#[derive(Clone)]
pub struct Tree<T> {
    /// Beginning of sequences.
    roots: Vec<Sequence<T>>,
}
