#[derive(Debug)]
pub struct Node<T> {
    parent: * const Node<T>,
    children: Vec<Box<Node<T>>>,
    value: T,
}

impl<T> Node<T> {

    pub(crate) fn new(value: T) -> Self {
        Self {
            parent: std::ptr::null(),
            children: Vec::new(),
            value,
        }
    }

    pub fn add_child(&mut self, child_data: T) -> &mut Node<T> {
        let mut child = Box::new(Node::new(child_data));
        child.parent = self as * const Node<T>;
        self.children.push(child);
        self.children.last_mut().unwrap()
    }

    pub fn remove_child(&mut self, index: usize) {
        let child = self.children.get_mut(index).unwrap();
        child.parent = std::ptr::null();
        self.children.remove(index);
    }

    pub fn child(&self, index: usize) -> &Node<T> {
        self.children.get(index).unwrap()
    }

    pub fn child_mut(&mut self, index: usize) -> &mut Node<T> {
        self.children.get_mut(index).unwrap()
    }

    pub fn parent(&self) -> Option<&Node<T>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&*self.parent) }
        }
    }

    pub fn parent_mut(&mut self) -> Option<&mut Node<T>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&mut *(self.parent as *mut Node<T>)) }
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
}