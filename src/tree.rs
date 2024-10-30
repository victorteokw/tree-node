use crate::node::Node;

#[derive(Debug)]
pub struct Tree<T> {
    root: Box<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new(value: T) -> Self {
        Self {
            root: Box::new(Node::new(value)),
        }
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.root
    }
}
