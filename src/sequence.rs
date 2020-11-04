use crate::Sequence;
use std::fmt;

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Sequence {
            sequence: Vec::new(),
            cursor: 0,
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

impl<T> Iterator for Sequence<T>
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
