#[allow(unused_imports)]
use crate::components::{node::*, bst::BST};

mod components;
fn main() {
}

#[cfg(test)]
mod tests {
    use std::process;

    use super::*;

    #[test]
    fn root_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);

        let root = bst_test.get_root().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

        assert_eq!(root, &5);
    }

    #[test]
    fn left_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        bst_test.add(Node::new(4));

        let root = bst_test.get_root().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });
        let left = bst_test.get_left().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

        assert_eq!(root, &5);
        assert_eq!(left, &4);
    }

    #[test]
    fn right_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        bst_test.add(Node::new(6));

        let root = bst_test.get_root().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });
        let right = bst_test.get_right().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

        assert_eq!(root, &5);
        assert_eq!(right, &6);
    }
}