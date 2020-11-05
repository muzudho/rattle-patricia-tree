pub mod linked_list;
pub mod sequence;
pub mod tree;
/*
use std::cell::RefCell;
use std::rc::Rc;
*/

/*
/// Linked List を作るには工夫が必要です。
/// [Rustでdoubly linked list](https://blog.ymgyt.io/entry/2019/08/17/013313)
///
/// `Rc` - 参照の所有者を複数にします。
/// `RefCell` - 参照先からでも値が変更できるようにします。
type SequenceRef<T> = Rc<RefCell<SequenceVal<Vec<T>>>>;

#[derive(Default)]
pub struct Tree<T> {
    /// Beginning of sequences.
    /// roots: Vec<SequenceRef<T>>,
    /// TODO Multiple head.
    /// 先頭ノードです。
    head: Option<SequenceRef<T>>,
    /// 末端ノードです。
    tail: Option<SequenceRef<T>>,
    /// ノード数です。
    length: usize,
}
*/

/*
#[derive(Debug)]
pub struct SequenceBuilder<T> {
    sequence: Vec<T>,
    prev: Option<SequenceRef<T>>,
    next: Option<SequenceRef<T>>,
    /// イテレーターで使います。
    length: usize,
}

#[derive(Debug)]
pub struct SequenceVal<T> {
    sequence: Vec<T>,
    /// イテレーターで使います。
    cursor: usize,
    prev: Option<SequenceRef<T>>,
    next: Option<SequenceRef<T>>,
    /// イテレーターで使います。
    length: usize,
}

/// Linked List で使います。
pub struct Iter<T> {
    current: Option<SequenceRef<T>>,
}
*/

/*
impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
*/
