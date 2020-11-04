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
    let seq3 = SequenceBuilder::default()
        .push(&vec!['I', 'l', 'l', 'u', 's', 't'])
        .build();
    let seq4 = SequenceBuilder::default()
        .push(&vec!['s', 't', 'a'])
        .build();
    let seq5 = SequenceBuilder::default()
        .push(&vec!['t', 'i', 'o', 'n'])
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

    // TODO 連結したい。
    tree.insert(&seq3);
    assert_eq!(
        "'H''e''l''l''o'','' ''W''o''r''l''d''!''!'
'I''l''l''u''s''t''r''a''t''i''o''n'
'I''l''l''u''s''t'
",
        format!("{:?}", tree)
    );

    // Concatination.
    let seq4and5 = SequenceBuilder::concat(&seq4, &seq5);
    assert_eq!("'s''t''a''t''i''o''n'", format!("{:?}", seq4and5));

    println!("Finished.");
}
