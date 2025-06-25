use b_tree::BPlusTree;

#[test]
fn insertion_and_searching() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert(i, i);
    }
    for i in 0..=100 {
        assert_eq!(tree.search(i), Some(i));
    }
}

#[test]
fn insertion_error() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert(i, i);
    }
    assert!(tree.insert(0, 0) == Err(b_tree::AlreadyExists));
}

#[test]
fn searching_error() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert(i, i);
    }
    assert!(tree.search(101).is_none());
}

#[test]
fn updating() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert(i, i);
    }
    for i in 0..=100 {
        let _ = tree.update(i, i + 1);
    }
    for i in 0..=100 {
        assert_eq!(tree.search(i), Some(i + 1));
    }
}

#[test]
fn updating_error() {
    let mut tree = BPlusTree::new(3);

    for i in 0..=100 {
        let _ = tree.insert(i, i);
    }
    assert!(tree.update(101, 1) == Err(b_tree::DoesNotExist));
}
