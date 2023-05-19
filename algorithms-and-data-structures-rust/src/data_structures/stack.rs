use super::singly_linked_node::SinglyLinkedNode;

#[derive(Debug, Clone)]
pub struct Stack<T>
{
    top: Option<SinglyLinkedNode<T>>,
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
        if let Some(mut top_node) = self.top.take()
        {
            let ret_data = top_node.data.clone();
            self.top = top_node.next.take().map(|node| *node);
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
