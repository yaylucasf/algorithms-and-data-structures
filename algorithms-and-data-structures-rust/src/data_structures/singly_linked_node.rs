use std::{cell::RefCell, borrow::BorrowMut};

#[derive(Debug, Clone)]
pub struct SinglyLinkedNode<T>
{
    pub data: T,
    pub next: Option<Box<SinglyLinkedNode<T>>>,
}

impl<'a, T> SinglyLinkedNode<T>
{
    pub fn new(data: T) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode { data, next: None }
    }

    pub fn new_with_next(data: T, next: SinglyLinkedNode<T>) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode {
            data,
            next: Some(Box::new(next)),
        }
    }

    pub fn get_next_data(&self) -> Option<&T>
    {
        self.next.as_ref().map(|x| &x.data)
    }

    pub fn get_next_data_mut(&mut self) -> Option<&mut T>
    {
        self.next.as_mut().map(|x| &mut x.data)
    }

    pub fn set_next_with_data(&mut self, data: T)
    {
        self.next = Some(Box::new(SinglyLinkedNode::new(data)));
    }

    pub fn get_nth(&self, n: usize) -> Option<&SinglyLinkedNode<T>>
    {
        let mut cur = self;

        for i in 0..n
        {
            if let Some(next) = cur.next.as_deref()
            {
                cur = next;
            }
            else
            {
                return None;
            }
        }

        Some(cur)
    }

    pub fn get_nth_mut(&mut self, n: usize) -> Option<&mut SinglyLinkedNode<T>>
    {
        let mut cur = self;

        for i in 0..n
        {
            if let Some(next) = cur.next.as_deref_mut()
            {
                cur = next;
            }
            else
            {
                return None;
            }
        }

        Some(cur)
    }

    pub fn insert(&mut self, other: SinglyLinkedNode<T>)
    {
        let old_next = self.next.take();
        self.next = Some(Box::new(other));
        self.next.as_mut().unwrap().next = old_next;
    }
}
