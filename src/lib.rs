pub mod sequence;
pub mod tree;

pub struct SequenceBuilder<T> {
    sequence: Vec<T>,
}

#[derive(Clone)]
pub struct SequenceVal<T> {
    sequence: Vec<T>,
    cursor: usize,
}

#[derive(Clone)]
pub struct Tree<T> {
    /// Beginning of sequences.
    roots: Vec<SequenceVal<T>>,
}
