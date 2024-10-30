use crate::mutable::Mutable;

#[derive(Debug)]
pub struct Node<T> {
    parent: * const Node<T>,
    children: Vec<Box<Node<T>>>,
    id: usize,
    value: T,
}

impl<T> Node<T> {

    pub(crate) fn new(parent: * const Node<T>, value: T, id: usize) -> Self {
        Self {
            parent,
            children: Vec::new(),
            value,
            id,
        }
    }

    pub fn add_child(&self, child_data: T) -> &Node<T> {
        let child = Box::new(Node::new(
            self as * const Node<T>,
            child_data,
            self.children.len(),
        ));
        unsafe { self.mut_self().children.push(child) };
        self.children.last().unwrap()
    }

    pub fn insert_child(&self, child_data: T, index: usize) -> &Node<T> {
        let child = Box::new(Node::new(
            self as * const Node<T>,
            child_data,
            index,
        ));
        unsafe { self.mut_self().children.insert(index, child) };
        self.increase_child_ids(index + 1, 1);
        self.children.get(index).unwrap()
    }

    pub unsafe fn remove_child(&self, index: usize) {
        let child = self.mut_self().children.get_mut(index).unwrap();
        child.parent = std::ptr::null();
        child.id = 0;
        self.mut_self().children.remove(index);
        self.decrease_child_ids(index, 1);
    }

    pub fn child(&self, index: usize) -> &Node<T> {
        self.children.get(index).unwrap()
    }

    pub unsafe fn child_mut(&self, index: usize) -> &mut Node<T> {
        self.mut_self().children.get_mut(index).unwrap()
    }

    pub fn parent(&self) -> Option<&Node<T>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&*self.parent) }
        }
    }

    pub unsafe fn parent_mut(&self) -> Option<&mut Node<T>> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&mut *(self.parent as *mut Node<T>)) }
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub unsafe fn value_mut(&self) -> &mut T {
        &mut self.mut_self().value
    }

    pub unsafe fn set_value(&self, value: T) {
        self.mut_self().value = value;
    }

    unsafe fn mut_self(&self) -> &mut Self {
        let container = Mutable::new(self as *const Node<T> as *mut Node<T>);
        container.as_ref(self)
    }

    fn increase_child_ids(&self, from_index: usize, by: usize) {
        let children = unsafe { &mut self.mut_self().children };
        let slice = &mut children[from_index..];
        slice.iter_mut().for_each(|child| child.id += by);
    }

    fn decrease_child_ids(&self, from_index: usize, by: usize) {
        let children = unsafe { &mut self.mut_self().children };
        let slice = &mut children[from_index..];
        slice.iter_mut().for_each(|child| child.id -= by);
    }
}