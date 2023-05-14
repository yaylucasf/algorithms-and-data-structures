use std::{fmt::Debug, cell::{RefCell, Cell}, borrow::Borrow};

use super::singly_linked_node::SinglyLinkedNode;

#[derive(Debug, Clone)]
pub struct SinglyLinkedList<T>
{
    head: Option<SinglyLinkedNode<T>>,
    size: usize
}

impl<T> SinglyLinkedList<T>
where T: Clone
{
    pub fn new() -> SinglyLinkedList<T>
    {
        SinglyLinkedList { head: None, size: 0 }
    }

    pub fn add(&mut self, elem: T)
    {
        if let Some(head) = &mut self.head
        {
            let edge = head.get_edge_node_mut();

            edge.set_next_data(elem);
        }
        else
        {
            self.head = Some(SinglyLinkedNode::new(elem));
        }
        self.size += 1;
    }

    pub fn get_head(&self) -> Option<&SinglyLinkedNode<T>>
    {
        self.head.as_ref()
    }

    pub fn get_head_mut(&mut self) -> Option<&mut SinglyLinkedNode<T>>
    {
        self.head.as_mut()
    }
}
