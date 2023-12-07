
#[derive(Debug)]
pub struct Node {
    data: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Node {
    pub fn new(data: i32) -> Self {
        Node {
            data: data,
            left: None,
            right: None,
        }
    }
    pub fn get_data(&self) -> &i32 {
        &self.data
    }
}