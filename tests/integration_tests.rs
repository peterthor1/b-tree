use b_tree::BPlusTree;

#[test]
fn insertion_and_deletion() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert_one(i, i);
    }
    for i in 0..=100 {
        assert_eq!(tree.search(i), Some(i));
    }
}

#[test]
fn updating() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert_one(i, i);
    }
    for i in 0..=100 {
        let _ = tree.update(i, i + 1);
    }
    for i in 0..=100 {
        assert_eq!(tree.search(i), Some(i + 1));
    }
}
