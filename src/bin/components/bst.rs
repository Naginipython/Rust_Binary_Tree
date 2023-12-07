use std::cmp::Ordering;

use crate::components::node::*;

#[derive(Debug)]
pub struct BST {
    root: Option<Node>,
}

#[allow(dead_code)]
impl BST {
    pub fn new() -> Self {
        BST {
            root: None,
        }
    }

    pub fn add(&mut self, node: Node) {
        fn helper(
            branch: &mut Option<Box<Node>>, 
            node: Node
        ) {
            if let Some(branch_node) = branch {
                match node.get_data().cmp(branch_node.get_data()) {
                    Ordering::Greater => helper(&mut branch_node.right, node),
                    Ordering::Less => helper(&mut branch_node.left, node),
                    _ => {}
                }
            } else {
                *branch = Some(Box::new(node));

            }
        }
        
        if let Some(root_node) = self.root.as_mut() {
            match node.get_data().cmp(root_node.get_data()) {
                Ordering::Greater => helper(&mut root_node.right, node),
                Ordering::Less => helper(&mut root_node.left, node),
                _ => {}
            }
        } else {
            self.root = Some(node);
        }
    }

    pub fn get_root(&self)-> &i32 {
        if let Some(root) = &self.root {
            root.get_data()
        } else {
            &-1
        }
    }

    pub fn get_left(&self) -> &i32 {
        if let Some(root) = &self.root {
            if let Some(branch) = &root.left {
                branch.get_data()
            } else {
                &-1
            }
        } else {
            &-1
        }
    }

    pub fn get_right(&self) -> &i32 {
        if let Some(root) = &self.root {
            if let Some(branch) = &root.right {
                branch.get_data()
            } else {
                &-1
            }
        } else {
            &-1
        }
    }
}