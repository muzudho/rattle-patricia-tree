pub mod sequence;
pub mod tree;

pub struct SequenceBuilder<T> {
    sequence: Vec<T>,
    head: Option<Box<SequenceVal<T>>>,
    tail: Option<Box<SequenceVal<T>>>,
}

#[derive(Clone)]
pub struct SequenceVal<T> {
    sequence: Vec<T>,
    cursor: usize,
    head: Option<Box<SequenceVal<T>>>,
    tail: Option<Box<SequenceVal<T>>>,
}

#[derive(Clone)]
pub struct Tree<T> {
    /// Beginning of sequences.
    roots: Vec<SequenceVal<T>>,
}
