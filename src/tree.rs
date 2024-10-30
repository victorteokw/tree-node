use std::ptr::null;
use crate::node::Node;

#[derive(Debug)]
pub struct Tree<T> {
    root: Box<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Self {
        Self {
            root: Box::new(Node::new(null(), value, 0)),
        }
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }
}
