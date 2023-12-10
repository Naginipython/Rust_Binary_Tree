
#[derive(Debug)]
pub struct Node<T: Ord + Clone> {
    data: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Ord + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }
    pub fn get_data(&self) -> &T {
        &self.data
    }
    pub fn get_data_cloned(&self) -> T {
        self.data.clone()
    }
}