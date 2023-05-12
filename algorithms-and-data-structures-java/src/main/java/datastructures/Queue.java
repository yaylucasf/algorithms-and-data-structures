package datastructures;

public class Queue<E> {

    private Node<E> front;
    private Node<E> back;

    private Integer size;

    public Queue() {

        size = 0;

    }

    public void clear() {

        front = null;
        back = null;

    }

    public E dequeue() {

        if (front == null && back == null) {

            return null;

        } else {

            var aux = front;

            if (front == back) {

                front = null;
                back = null;

            } else {

                front = front.getNextNode();

            }

            size -= 1;

            return aux.getElement();

        }

    }

    public Boolean empty() {

        return front == null && back == null;

    }

    public void enqueue(E element) {

        if (front == null && back == null) {

            front = new Node<E>(element);
            back = front;

        } else {

            back.setNextNode(new Node<E>(element));
            back = back.getNextNode();

        }

        size += 1;

    }

    public E peek() {

        return front == null ? null : front.getElement();

    }

}
