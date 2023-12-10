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

    #[test]
    fn test_to_vec_simple() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        bst_test.add(Node::new(6));

        let vec = bst_test.to_vec();
        assert_eq!(vec, vec![5, 6]);
    }

    #[test]
    fn test_to_vec_hard() {
        let node = Node::new(5);
        let mut bst_test = BST::new();

        bst_test.add(node);
        bst_test.add(Node::new(6));
        bst_test.add(Node::new(3));
        bst_test.add(Node::new(21));
        bst_test.add(Node::new(9));
        bst_test.add(Node::new(8));
        bst_test.add(Node::new(32));
        bst_test.add(Node::new(91));
        bst_test.add(Node::new(68));
        bst_test.add(Node::new(20));
        bst_test.add(Node::new(75));
        bst_test.add(Node::new(109));

        let vec = bst_test.to_vec();

        assert_eq!(vec, vec![3,5,6,8,9,20,21,32,68,75,91,109]);

        let root = bst_test.get_root().unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });

        //bst persists
        assert_eq!(root, &5);
    }
}