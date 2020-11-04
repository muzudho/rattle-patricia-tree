use crate::Sequence;
use std::fmt;

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Sequence {
            sequence: Vec::new(),
        }
    }
}

impl<T> Sequence<T> {
    pub fn append(&mut self, raw: &Vec<T>) {}
}

impl<T> fmt::Debug for Sequence<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // TODO
        buf.push_str(&format!("WIP. "));
        write!(f, "{}", buf)
    }
}
