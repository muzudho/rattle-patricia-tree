use crate::Sequence;
use crate::Tree;
use std::fmt;

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree { root: None }
    }
}

impl<T> Tree<T>
where
    T: std::clone::Clone,
{
    pub fn insert(&mut self, seq: &Sequence<T>) {
        self.root = Some(seq.clone());
    }
}

impl<T> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // TODO
        buf.push_str(&format!("WIP. "));
        write!(f, "{}", buf)
    }
}
