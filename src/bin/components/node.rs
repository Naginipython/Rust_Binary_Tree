
#[derive(Debug)]
pub struct Node<T: Ord> {
    data: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Ord> Node<T> {
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
}