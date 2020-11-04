use crate::{SequenceBuilder, SequenceVal};
use std::fmt;

impl<T> Default for SequenceBuilder<T> {
    fn default() -> Self {
        SequenceBuilder {
            sequence: Vec::new(),
        }
    }
}

impl<T> Default for SequenceVal<T> {
    fn default() -> Self {
        SequenceVal {
            sequence: Vec::new(),
            cursor: 0,
        }
    }
}

impl<T> SequenceBuilder<T>
where
    T: std::clone::Clone,
{
    pub fn build(&self) -> SequenceVal<T> {
        SequenceVal {
            sequence: self.sequence.clone(),
            cursor: 0,
        }
    }

    pub fn push<'a>(&'a mut self, raw: &Vec<T>) -> &'a Self {
        self.sequence.extend(raw.clone());
        self
    }
}

impl<T> Iterator for SequenceVal<T>
where
    T: std::clone::Clone,
{
    type Item = T;
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<T> {
        if self.cursor < self.sequence.len() {
            let item = Some(self.sequence[self.cursor].clone());
            self.cursor += 1;
            return item;
        }

        return None;
    }
}

impl<T> fmt::Debug for SequenceVal<T>
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
