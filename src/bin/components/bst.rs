use std::cmp::Ord;
use std::cmp::Ordering;
use std::error::Error;

use crate::components::node::*;

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Option<Node<T>>,
}

#[allow(dead_code)]
impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST {
            root: None,
        }
    }

    pub fn add(&mut self, node: Node<T>) {
        fn helper<T: Ord>(
            branch: &mut Option<Box<Node<T>>>, 
            node: Node<T>
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

    pub fn get_root(&self)-> Result<&T, Box<dyn Error>> {
        if let Some(root) = &self.root {
            Ok(root.get_data())
        } else {
            Err("Could not get root")?
        }
    }

    pub fn get_left(&self) -> Result<&T, Box<dyn Error>> {
        if let Some(root) = &self.root {
            if let Some(branch) = &root.left {
                Ok(branch.get_data())
            } else {
                Err("Could not get left")?
            }
        } else {
            Err("Could not get root")?
        }
    }

    pub fn get_right(&self) -> Result<&T, Box<dyn Error>> {
        if let Some(root) = &self.root {
            if let Some(branch) = &root.right {
                Ok(branch.get_data())
            } else {
                Err("Could not get left")?
            }
        } else {
            Err("Could not get root")?
        }
    }
}