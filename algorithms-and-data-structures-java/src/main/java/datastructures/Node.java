package datastructures;

class Node<E> {

    private E element;
    private Node<E> nextNode;

    Node(E element) {

        this.element = element;

    }

    E getElement() {

        return element;

    }

    Node<E> getNextNode() {

        return nextNode;

    }

    void setNextNode(Node<E> node) {

        nextNode = node;

    }

}
