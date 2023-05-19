use algorithms_and_data_structures_rust::data_structures::stack::Stack;

#[test]
fn stack_pop()
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
