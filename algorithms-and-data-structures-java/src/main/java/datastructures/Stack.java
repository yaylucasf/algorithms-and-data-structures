package datastructures;

public class Stack<E> {

    private Node<E> top;

    public void add(E element) {

        if (top == null) {

            top = new Node<E>(element);

        } else {

            var aux = new Node<E>(element);
            aux.setNextNode(top);
            top = aux;

        }

    }

    public Boolean empty() {

        return top == null;

    }

    public E peek() {

        return top == null ? null : top.getElement();

    }

    public E pop() {

        if (top == null) {

            return null;

        } else {

            var aux = top;
            top = top.getNextNode();

            return aux.getElement();

        }

    }

}
