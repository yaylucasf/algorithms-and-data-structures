package datastructures;

public class LinkedList<E> {

    private Node<E> front;
    private Node<E> back;
    private Integer size;

    public LinkedList() {

        size = 0;

    }

    public void addFirst(E element) {

        if (front == null && back == null) {

            front = new Node<E>(element);
            back = front;

        } else {

            var aux = new Node<E>(element);
            aux.setNextNode(front);
            front = aux;

        }

        size += 1;

    }

    public void addLast(E element) {

        if (front == null && back == null) {

            front = new Node<E>(element);
            back = front;

        } else {

            back.setNextNode(new Node<E>(element));
            back = back.getNextNode();

        }

        size += 1;

    }

    public void clear() {

        front = null;
        back = null;

    }

    public Boolean empty() {

        return front == null && back == null;

    }

    public E getFrontElement() {

        return front == null ? null : front.getElement();

    }

    public E getBackElement() {

        return back == null ? null : back.getElement();

    }

    public Integer getSize() {

        return size == null ? 0 : size;

    }

}
