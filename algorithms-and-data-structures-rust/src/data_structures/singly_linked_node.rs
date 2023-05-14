#[derive(Debug, Clone)]
pub struct SinglyLinkedNode<T>
{
    pub data: T,
    pub next: Option<Box<SinglyLinkedNode<T>>>
}

impl<T> SinglyLinkedNode<T>
{
    pub fn new(data: T) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode { data, next: None }
    }

    pub fn new_with_next(data: T, next: SinglyLinkedNode<T>) -> SinglyLinkedNode<T>
    {
        SinglyLinkedNode { data, next: Some(Box::new(next)) }
    }

    pub fn get_next_data(&self) -> Option<&T>
    {
        self.next.as_ref().map(|x| {
            &x.data
        })
    }

    pub fn get_next_data_mut(&mut self) -> Option<&mut T>
    {
        self.next.as_mut().map(|x| {
            &mut x.data
        })
    }

    pub fn set_next_with_data(&mut self, data: T)
    {
        self.next = Some(Box::new(SinglyLinkedNode::new(data)));
    }

    // TODO: implement iterator so this can be removed
    pub fn get_edge_node(&self) -> &SinglyLinkedNode<T>
    {
        let mut cur = self as *const SinglyLinkedNode<T>;

        unsafe
        {
            while let Some(next) = &(*cur).next
            {
                let a = next.as_ref() as *const SinglyLinkedNode<T>;
                cur = a;
            }
            cur.as_ref().unwrap()
        }
    }

    pub fn get_edge_node_mut(&mut self) -> &mut SinglyLinkedNode<T>
    {
        let mut cur = self as *mut SinglyLinkedNode<T>;

        unsafe
        {
            while let Some(next) = &mut (*cur).next
            {
                let a = next.as_mut() as *mut SinglyLinkedNode<T>;
                cur = a;
            }
            cur.as_mut().unwrap()
        }
    }
}
