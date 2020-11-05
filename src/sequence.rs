use crate::{Link, SequenceBuilder, SequenceVal};
use std::fmt;
use std::rc::Rc;

impl<T> Default for SequenceBuilder<T> {
    fn default() -> Self {
        SequenceBuilder {
            sequence: Vec::new(),
            prev: None,
            next: None,
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
            prev: self.prev.clone(),
            next: self.next.clone(),
        }
    }

    /// 2つのシーケンスを結合して、１つのシーケンスを作成します。  
    /// ただし、 headのtail と、 tailのhead は None である必要があります。  
    pub fn concat(head: &SequenceVal<T>, tail: &SequenceVal<T>) -> SequenceVal<T> {
        if let Some(_) = head.next {
            panic!("head.tail is not None.");
        }
        if let Some(_) = tail.prev {
            panic!("tail.head is not None.");
        }

        let mut buf = Vec::new();
        for x in head.clone().into_iter() {
            buf.push(x);
        }
        for x in tail.clone().into_iter() {
            buf.push(x);
        }
        SequenceVal {
            sequence: buf,
            cursor: 0,
            prev: head.prev.clone(),
            next: tail.next.clone(),
        }
    }

    /// 2つのシーケンスを結合して、１つのシーケンスを作成します。  
    /// ただし、 firstのtail と、 secondのhead は None である必要があります。  
    pub fn link(first: &Link<SequenceVal<T>>, second: &Link<SequenceVal<T>>) {
        // `borrow_mut()` - 参照を、同時に１つだけ、変更可能にします。
        if let Some(_) = first.borrow_mut().next {
            panic!("first.next is not None.");
        }
        if let Some(_) = second.borrow_mut().prev {
            panic!("second.prev is not None.");
        }

        // firstのnext を second にします。
        // secondのprev を first にします。
        // `RC::clone( )` - 所有者が増えました。
        first.borrow_mut().next = Some(Rc::clone(second));
        second.borrow_mut().prev = Some(Rc::clone(first));
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
