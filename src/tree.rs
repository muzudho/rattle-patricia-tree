use crate::Sequence;
use crate::Tree;
use std::fmt;

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree { roots: Vec::new() }
    }
}

impl<T> Tree<T>
where
    T: std::clone::Clone,
{
    pub fn insert(&mut self, seq: &Sequence<T>) {
        self.roots.push(seq.clone());
    }
}

impl<T> fmt::Debug for Tree<T>
where
    T: std::fmt::Debug + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // Multiple root.
        // TODO clone しまくってて重そうだが他に方法はないか？
        for root in self.roots.iter() {
            // println!("(trace.29) chr={:?}", root);
            for chr in root.clone().into_iter() {
                // println!("(trace.31) chr={:?}", chr);
                buf.push_str(&format!("{:?}", chr));
            }
        }
        write!(f, "{}", buf)
    }
}
