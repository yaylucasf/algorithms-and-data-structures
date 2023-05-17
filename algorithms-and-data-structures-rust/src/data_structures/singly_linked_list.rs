use std::{
    borrow::Borrow,
    cell::{Cell, RefCell},
    fmt::Debug,
};

use super::singly_linked_node::SinglyLinkedNode;

#[derive(Debug, Clone)]
pub struct SinglyLinkedList<T>
{
    head: Option<SinglyLinkedNode<T>>,
}

impl<'a, T> SinglyLinkedList<T>
{
    pub fn new() -> SinglyLinkedList<T>
    {
        SinglyLinkedList { head: None }
    }

    pub fn add(&mut self, elem: T)
    {
        if let Some(head) = &mut self.head
        {
            let edge = head.iter_mut().last().unwrap();

            edge.set_next_with_data(elem);
        }
        else
        {
            self.head = Some(SinglyLinkedNode::new(elem));
        }
    }

    pub fn get_head(&self) -> Option<&SinglyLinkedNode<T>>
    {
        self.head.as_ref()
    }

    pub fn get_head_mut(&mut self) -> Option<&mut SinglyLinkedNode<T>>
    {
        self.head.as_mut()
    }

    pub fn iter(&'a self) -> SinglyLinkedListIterator<'a, T>
    {
        SinglyLinkedListIterator {
            next: self.head.as_ref(),
        }
    }

    pub fn iter_mut(&'a mut self) -> SinglyLinkedListMutIterator<'a, T>
    {
        SinglyLinkedListMutIterator {
            next: self.head.as_mut(),
        }
    }
}

pub struct SinglyLinkedListIterator<'a, T>
{
    next: Option<&'a SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedListIterator<'a, T>
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

pub struct SinglyLinkedListMutIterator<'a, T>
{
    next: Option<&'a mut SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedListMutIterator<'a, T>
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

pub struct SinglyLinkedListOwnedIterator<T>
{
    next: Option<SinglyLinkedNode<T>>,
}

impl<T> Iterator for SinglyLinkedListOwnedIterator<T>
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

impl<T> IntoIterator for SinglyLinkedList<T>
{
    type Item = T;
    type IntoIter = SinglyLinkedListOwnedIterator<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        SinglyLinkedListOwnedIterator { next: self.head }
    }
}
