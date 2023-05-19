use super::singly_linked_node::SinglyLinkedNode;

#[derive(Debug, Clone)]
pub struct Stack<T>
{
    top: Option<SinglyLinkedNode<T>>,
    size: usize,
}

impl<'a, T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Stack<T>
    {
        Stack { top: None, size: 0 }
    }

    pub fn add(&mut self, elem: T)
    {
        if let Some(top_node) = self.top.take()
        {
            let mut new_top = SinglyLinkedNode::new(elem);
            new_top.next = Some(Box::new(top_node));

            self.top = Some(new_top);
        }
        else
        {
            self.top = Some(SinglyLinkedNode::new(elem));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    {
        self.top.take().map(|mut top_node| {
            let ret_data = top_node.data.clone();
            self.top = top_node.next.take().map(|node| *node);
            self.size -= 1;

            ret_data
        })
    }

    pub fn peek(&self) -> Option<T>
    {
        self.top.as_ref().map(|top_node| {
            unsafe { std::ptr::read(&top_node.data) }
        })
    }

    pub fn get_top(&self) -> Option<&SinglyLinkedNode<T>>
    {
        self.top.as_ref()
    }

    pub fn get_top_mut(&mut self) -> Option<&mut SinglyLinkedNode<T>>
    {
        self.top.as_mut()
    }

    pub fn iter(&'a self) -> StackIterator<'a, T>
    {
        StackIterator {
            next: self.top.as_ref(),
        }
    }

    pub fn iter_mut(&'a mut self) -> StackMutIterator<'a, T>
    {
        StackMutIterator {
            next: self.top.as_mut(),
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.size == 0
    }

    pub fn get_size(&self) -> usize
    {
        self.size
    }
}

pub struct StackIterator<'a, T>
{
    next: Option<&'a SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for StackIterator<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

pub struct StackMutIterator<'a, T>
{
    next: Option<&'a mut SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for StackMutIterator<'a, T>
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

pub struct StackOwnedIterator<T>
{
    next: Option<SinglyLinkedNode<T>>,
}

impl<T> Iterator for StackOwnedIterator<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.next.take().map(|node| {
            self.next = node.next.map(|node| *node);
            node.data
        })
    }
}

impl<T> IntoIterator for Stack<T>
{
    type Item = T;
    type IntoIter = StackOwnedIterator<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        StackOwnedIterator { next: self.top }
    }
}
