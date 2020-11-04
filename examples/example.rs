extern crate rattle_tree;

use rattle_tree::{SequenceBuilder, Tree};

fn main() {
    println!("Start.");

    // Sequence.
    let seq1 = SequenceBuilder::default()
        .push(&vec![
            'H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!', '!',
        ])
        .build();
    let seq2 = SequenceBuilder::default()
        .push(&vec![
            'I', 'l', 'l', 'u', 's', 't', 'r', 'a', 't', 'i', 'o', 'n',
        ])
        .build();

    // Tree.
    let mut tree = Tree::default();
    tree.insert(&seq1);
    // Hello, World!!
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
",
        format!("{:?}", tree)
    );

    tree.insert(&seq2);
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
'I''l''l''u''s''t''r''a''t''i''o''n'
",
        format!("{:?}", tree)
    );

    println!("Finished.");
}
