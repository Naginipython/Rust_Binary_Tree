#[allow(dead_code)]

pub struct BST {
    pub data: Option<i32>,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>,
}

#[allow(dead_code)]
impl BST {
    pub fn new() -> Self {
        BST {
            data: None,
            left: None,
            right: None
        }
    }
    
    fn create(data: i32) -> Self {
        BST {
            data: Some(data),
            left: None,
            right: None,
        }
    }   

    pub fn add(&mut self, data: i32) {
        fn helper(branch: &mut Option<Box<BST>>, data: i32) {
            if let Some(node) = branch {
                if let Some(node_data) = &node.data {
                    if data < *node_data {
                        helper(&mut node.left, data);
                    } else if data > *node_data {
                        helper(&mut node.right, data);
                    }
                }
            } else {
                *branch = Some(Box::new(BST::create(data)));
            }
        }
        if let Some(node_data) = &self.data {
            if data < *node_data {
                helper(&mut self.left, data);
            } else if data > *node_data {
                helper(&mut self.right, data);
            }
        } else {
            self.data = Some(data);
        }
    }
    fn print(&self) {
        if let Some(data) = &self.data {
            println!("{}", data);
        }
    }
    fn print_left(&self) {
        if let Some(node) = &self.left {
            if let Some(data) = &node.data {
                println!("{}", data);
            }
        }
    }
    fn print_right(&self) {
        if let Some(node) = &self.right {
            if let Some(data) = &node.data {
                println!("{}", data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_works() {
        let mut test = BST::new();
        test.add(5);
        // test.print();
        assert_eq!(Some(5), test.data);
    }

    #[test]
    fn left_works() {
        let mut test = BST::new();
        test.add(5);
        // test.print();
        test.add(4);
        // test.print_left()
        assert_eq!(Some(5), test.data);
        assert_eq!(Some(4), test.left.unwrap().data);
    }

    #[test]
    fn right_works() {
        let mut test = BST::new();
        test.add(5);
        // test.print();
        test.add(6);
        test.print_right();
        assert_eq!(Some(5), test.data);
        assert_eq!(Some(6), test.right.unwrap().data);
    }
}