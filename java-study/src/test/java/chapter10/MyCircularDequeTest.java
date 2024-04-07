package chapter10;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class MyCircularDequeTest {
    @Test
    public void test() {
        MyCircularDeque deque = new MyCircularDeque(3);

        Assertions.assertTrue(deque.insertLast(1));
        Assertions.assertTrue(deque.insertLast(2));

        Assertions.assertTrue(deque.insertFront(3));
        Assertions.assertFalse(deque.insertFront(4));

        Assertions.assertEquals(deque.getRear(), 2);

        Assertions.assertTrue(deque.isFull());

        Assertions.assertTrue(deque.deleteLast());
        Assertions.assertTrue(deque.deleteFront());

        Assertions.assertTrue(deque.insertFront(4));

        Assertions.assertEquals(deque.getFront(), 4);
    }
}
