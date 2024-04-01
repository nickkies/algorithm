package chapter09;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class MyQueueTest {
    @Test
    public void test() {
        MyQueue<Integer> queue = new MyQueue<>();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        Assertions.assertEquals(queue.peek(), 1);
        Assertions.assertEquals(queue.pop(), 1);
        Assertions.assertEquals(queue.pop(), 2);
        Assertions.assertFalse(queue.empty());

        queue.pop();

        Assertions.assertTrue(queue.empty());

    }
}
