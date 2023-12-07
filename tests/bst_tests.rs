use binary_tree;

#[test]
fn test_the_test() {
    let n = 2;
    assert_eq!(n, 2);
    let mut test = binary_tree::BST::new();
    test.add(5);
    assert_eq!(Some(5), test.data);
}

// Majority of tests are within lib.rs and node_based