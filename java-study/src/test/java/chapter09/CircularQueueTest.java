package chapter09;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class CircularQueueTest {
    @Test
    public void test() {
        CircularQueue queue = new CircularQueue(5);

        Assertions.assertEquals(queue.enQueueForce(10), -1);
        Assertions.assertTrue(queue.enQueue(20));
        Assertions.assertTrue(queue.enQueue(30));
        Assertions.assertTrue(queue.enQueue(40));

        Assertions.assertEquals(queue.rear(), 40);
        Assertions.assertFalse(queue.isFull());

        Assertions.assertTrue(queue.deQueue());
        Assertions.assertTrue(queue.deQueue());

        Assertions.assertTrue(queue.enQueue(50));
        Assertions.assertTrue(queue.enQueue(60));

        Assertions.assertEquals(queue.rear(), 60);

        Assertions.assertTrue(queue.enQueue(70));

        Assertions.assertTrue(queue.isFull());
        Assertions.assertEquals(queue.front(), 30);

        // 무한으로 입력 가능한 경우 테스트 시작
        Assertions.assertFalse(queue.enQueue(80));
        Assertions.assertEquals(queue.enQueueForce(80), 30);

        Assertions.assertEquals(queue.rear(), 80);
        Assertions.assertEquals(queue.front(), 40);

        Assertions.assertTrue(queue.deQueue());
        Assertions.assertTrue(queue.deQueue());
        Assertions.assertTrue(queue.deQueue());
        Assertions.assertTrue(queue.deQueue());
        Assertions.assertTrue(queue.deQueue());

        Assertions.assertTrue(queue.isEmpty());

        Assertions.assertFalse(queue.deQueue());

        Assertions.assertTrue(queue.enQueue(90));
        Assertions.assertTrue(queue.enQueue(100));

        Assertions.assertEquals(queue.front(), 90);
        Assertions.assertEquals(queue.rear(), 100);
    }
}
