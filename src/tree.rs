use crate::Tree;

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree { root: None }
    }
}

impl<T> Tree<T> {
    pub fn insert_all(sequence: &Vec<T>) {}
}
