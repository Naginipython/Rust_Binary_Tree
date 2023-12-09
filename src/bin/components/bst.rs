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
            match branch {
                Some(branch_node) =>
                    match node.get_data().cmp(branch_node.get_data()) {
                        Ordering::Greater => helper(&mut branch_node.right, node),
                        Ordering::Less => helper(&mut branch_node.left, node),
                        _ => {}
                    },
                None => *branch = Some(Box::new(node))
            }
        }
        
        match self.root.as_mut() {
            Some(root_node) => // Compares node to to root's data
                match node.get_data().cmp(root_node.get_data()) {
                    Ordering::Greater => helper(&mut root_node.right, node),
                    Ordering::Less => helper(&mut root_node.left, node),
                    _ => {}
                },
            None => self.root = Some(node)
        }
    }

    pub fn get_root(&self)-> Result<&T, Box<dyn Error>> {
        match &self.root {
            Some(root) => Ok(root.get_data()),
            None => Err("Could not get root")?
        }
    }

    pub fn get_left(&self) -> Result<&T, Box<dyn Error>> {
        match &self.root {
            Some(root) => 
                match &root.left {
                    Some(branch) => Ok(branch.get_data()),
                    None => Err("Could not get left")?
                },
            None => Err("Could not get root")?
        }
    }

    pub fn get_right(&self) -> Result<&T, Box<dyn Error>> {
        match &self.root {
            Some(root) => 
                match &root.right {
                    Some(branch) => Ok(branch.get_data()),
                    None => Err("Could not get right")?
                },
            None => Err("Could not get root")?
        }
    }
}