import datastructures.Queue;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class QueueTest {

    @Test
    public void clearTest() {

        var queue = new Queue<Integer>();

        queue.clear();

        Assertions.assertEquals(true, queue.empty());

    }

    @Test
    public void addAndPopTest() {

        var queue = new Queue<Integer>();

        for (int i = 0; i < 100; i++) {

            queue.enqueue(i);

        }

        for (int i = 0; i < 100; i++) {

            Assertions.assertEquals(i, queue.dequeue());

        }

    }

    @Test
    public void addAndPeekTest() {

        var queue = new Queue<Integer>();

        for (int i = 0; i < 100; i++) {

            queue.enqueue(i);

        }

        for (int i = 0; i < 100; i++) {

            Assertions.assertEquals(i, queue.peek());
            queue.dequeue();

        }

    }

}
