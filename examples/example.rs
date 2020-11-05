extern crate rattle_tree;

// use rattle_tree::{SequenceBuilder, Tree};
use rattle_tree::linked_list::LinkedList;

fn main() {
    println!("Start.");

    let mut list: LinkedList<i32> = LinkedList::new();
    list.append(0);
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);
    list.append(5);
    list.append(6);
    list.append(7);
    list.append(8);
    list.append(9);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), Some(0));
    assert_eq!(iter.next_back(), None);

    println!("Finished.");
    /*
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
        tree.insert(SequenceBuilder::build_ref(&seq1));
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
    */
}
