#[allow(unused_imports)]
use crate::components::{node::*, bst::BST};

mod components;
fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        assert_eq!(bst_test.get_root(), &5);
    }

    #[test]
    fn left_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        bst_test.add(Node::new(4));

        assert_eq!(bst_test.get_root(), &5);
        assert_eq!(bst_test.get_left(), &4);
    }

    #[test]
    fn right_works() {
        let node = Node::new(5);
        let mut bst_test = BST::new();
        bst_test.add(node);
        bst_test.add(Node::new(6));

        assert_eq!(bst_test.get_root(), &5);
        assert_eq!(bst_test.get_right(), &6);
    }
}