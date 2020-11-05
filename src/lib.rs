pub mod sequence;
pub mod tree;
use std::cell::RefCell;
use std::rc::Rc;

/// Linked List を作るには工夫が必要です。
/// [Rustでdoubly linked list](https://blog.ymgyt.io/entry/2019/08/17/013313)
///
/// `Rc` - 参照の所有者を複数にします。
/// `RefCell` - ランタイム時に同時に１つの参照先だけ値が変更できるようにします。
/// だいたい Box<> の感覚で使う事ができます。
type Link<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct Tree<T> {
    /// Beginning of sequences.
    roots: Vec<SequenceVal<T>>,
}

pub struct SequenceBuilder<T> {
    sequence: Vec<T>,
    prev: Option<Link<SequenceVal<T>>>,
    next: Option<Link<SequenceVal<T>>>,
}

#[derive(Clone)]
pub struct SequenceVal<T> {
    sequence: Vec<T>,
    cursor: usize,
    prev: Option<Link<SequenceVal<T>>>,
    next: Option<Link<SequenceVal<T>>>,
}
