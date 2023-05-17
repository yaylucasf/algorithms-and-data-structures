use super::singly_linked_node::SinglyLinkedNode;

#[derive(Debug, Clone)]
pub struct Stack<T>
{
    top: Option<Box<SinglyLinkedNode<T>>>,
    size: usize,
}

impl<T> Stack<T>
where
    T: Clone,
{
    pub fn new() -> Stack<T>
    {
        Stack { top: None, size: 0 }
    }

    // TODO: potentially make add and pop use unsafe methods for performance
    pub fn add(&mut self, elem: T)
    {
        if let Some(top_node) = &self.top
        {
            let mut new_top = SinglyLinkedNode::new(elem);
            new_top.next = Some(top_node.clone());

            self.top = Some(Box::new(new_top));
        }
        else
        {
            self.top = Some(Box::new(SinglyLinkedNode::new(elem)));
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    {
        if let Some(top_node) = &mut self.top
        {
            let ret_data = top_node.data.clone();
            self.top = top_node.next.clone();
            self.size -= 1;

            Some(ret_data)
        }
        else
        {
            None
        }
    }

    pub fn peek(&self) -> Option<T>
    {
        if let Some(top_node) = &self.top
        {
            Some(unsafe { std::ptr::read(&top_node.data) })
        }
        else
        {
            None
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
