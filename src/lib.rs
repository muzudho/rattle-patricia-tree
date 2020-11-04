pub mod sequence;
pub mod tree;

#[derive(Clone)]
pub struct Sequence<T> {
    sequence: Vec<T>,
}

#[derive(Clone)]
pub struct Tree<T> {
    // TODO multiple root
    root: Option<Sequence<T>>,
}
