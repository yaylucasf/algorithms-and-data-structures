use algorithms_and_data_structures_rust::data_structures::stack::Stack;

#[test]
fn stack_push_pop()
{
    let mut stack = Stack::new();

    for i in 0..100
    {
        stack.add(i);
    }

    for i in 100..0
    {
        assert_eq!(stack.pop(), Some(i));
    }
}

#[test]
fn stack_push_peek()
{
    let mut stack = Stack::new();

    for i in 0..100
    {
        stack.add(i);
    }

    for i in 100..0
    {
        assert_eq!(stack.peek(), Some(i));
        stack.pop();
    }
}

#[test]
fn stack_iter()
{
    let mut stack = Stack::new();

    stack.add(2);
    stack.add(1);
    stack.add(0);

    for (i, data) in stack.iter().enumerate()
    {
        assert_eq!(i, *data);
    }
}

#[test]
fn stack_iter_mut()
{
    let mut stack = Stack::new();

    stack.add(3);
    stack.add(2);
    stack.add(1);

    for (i, data) in stack.iter_mut().enumerate()
    {
        *data -= 1;
        assert_eq!(i, *data);
    }
}

#[test]
fn stack_into_iter()
{
    let mut stack = Stack::new();

    stack.add(2);
    stack.add(1);
    stack.add(0);

    let vec: Vec<usize> = stack.into_iter().map(|x| x).collect();
    assert_eq!(vec, [0, 1, 2]);
}
