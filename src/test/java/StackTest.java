import datastructures.Stack;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class StackTest {

    @Test
    public void clearTest() {

        var stack = new Stack<Integer>();

        stack.clear();

        Assertions.assertEquals(true, stack.empty());

    }

    @Test
    public void addAndPopTest() {

        var stack = new Stack<Integer>();

        for (int i = 0; i < 100; i++) {

            stack.add(i);

        }

        for (int i = 100 - 1; i >= 0; i--) {

            Assertions.assertEquals(i, stack.pop());

        }

    }

    @Test
    public void addAndPeekTest() {

        var stack = new Stack<Integer>();

        for (int i = 0; i < 100; i++) {

            stack.add(i);

        }

        for (int i = 100 - 1; i >= 0; i--) {

            Assertions.assertEquals(i, stack.peek());
            stack.pop();

        }

    }

}
