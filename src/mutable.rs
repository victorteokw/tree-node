use crate::node::Node;

pub(crate) struct Mutable<T> {
    pub(crate) value: * mut Node<T>
}

impl<T> Mutable<T> {
    pub(crate) fn new(value: * mut Node<T>) -> Self {
        Self { value }
    }

    pub(crate) fn as_ref<'a>(&self, _value: &'a Node<T>) -> &'a mut Node<T> {
        unsafe { &mut *self.value }
    }
}