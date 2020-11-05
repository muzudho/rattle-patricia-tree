/*
use crate::{Iter, SequenceBuilder, SequenceVal};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

impl<T> Default for SequenceBuilder<T> {
    fn default() -> Self {
        SequenceBuilder {
            sequence: Vec::new(),
            prev: None,
            next: None,
            length: 0,
        }
    }
}

impl<T> SequenceBuilder<T>
where
    T: std::clone::Clone,
{
    pub fn build(&self) -> Rc<RefCell<SequenceVal<T>>> {
        Rc::new(RefCell::new(SequenceVal {
            sequence: self.sequence.clone(),
            cursor: 0,
            prev: self.prev.clone(),
            next: self.next.clone(),
            length: 0,
        }))
    }
*/
/*
/// Reference.
pub fn build_ref(val: &SequenceVal<T>) -> Rc<RefCell<SequenceVal<T>>> {
    Rc::new(RefCell::new(*val))
}
*/
/*
    /// 2つのシーケンスを結合して、１つのシーケンスを作成します。
    /// ただし、 headのtail と、 tailのhead は None である必要があります。
    pub fn concat(head: &SequenceVal<T>, tail: &SequenceVal<T>) -> SequenceVal<T> {
        if let Some(_) = head.next {
            panic!("head.tail is not None.");
        }
        if let Some(_) = tail.prev {
            panic!("tail.prev is not None.");
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
            length: 0,
        }
    }
*/
/*
/// 2つのシーケンスをリンクで結合します。
/// ただし、入力時は、 headのtail と、 tailのhead は None である必要があります。
pub fn link(head: &mut SequenceVal<T>, tail: &SequenceVal<T>) {
    // head の tail に tail をリンクします。
    match head.tail.take() {
        Some(head_tail) => {
            panic!("Unimplemented"); // TODO
        }
        None => {
            // first element
            // Rc を使っているなら、クローンは Rc::clone() を使ってください。
            head.tail = Some(Rc::clone(tail));
        }
    }

    // TODO tail.head = Some(Rc::clone(&head));
}
*/
/*
    pub fn push<'a>(&'a mut self, raw: &Vec<T>) -> &'a Self {
        self.sequence.extend(raw.clone());
        self
    }

    /// イテレーターで使います。
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            current: if self.len() == 0 {
                None
            } else {
                Some(Rc::clone(&self.prev.as_ref().unwrap()))
            },
        }
    }
}

impl<T> SequenceVal<T> {
    /// イテレーターで使います。
    pub fn len(&self) -> usize {
        self.length
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
*/
