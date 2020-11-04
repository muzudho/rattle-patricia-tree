use crate::Sequence;
use std::fmt;

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Sequence {
            sequence: Vec::new(),
        }
    }
}

impl<T> Sequence<T>
where
    T: std::clone::Clone,
{
    pub fn append(&mut self, raw: &Vec<T>) {
        self.sequence.extend(raw.clone());
    }
}

impl<T> fmt::Debug for Sequence<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for chr in &self.sequence {
            buf.push_str(&format!("{:?}", chr));
        }
        write!(f, "{}", buf)
    }
}
