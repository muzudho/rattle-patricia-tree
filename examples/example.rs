extern crate rattle_tree;

use rattle_tree::{Sequence, Tree};

fn main() {
    println!("Start.");

    // Sequence.
    let mut seq = Sequence::default();
    seq.append(&vec![
        'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
    ]);

    // Tree.
    let mut tree = Tree::default();
    tree.insert(&seq);
    assert_eq!("Hello, World!!", format!("tree={:?}", tree));

    println!("Finished.");
}
