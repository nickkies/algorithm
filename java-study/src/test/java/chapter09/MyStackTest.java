package chapter09;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class MyStackTest {
    @Test
    public void test() {
        MyStack<Integer> stack = new MyStack<>();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        Assertions.assertEquals(stack.top(), 3);
        Assertions.assertEquals(stack.pop(), 3);
        Assertions.assertEquals(stack.pop(), 2);
        Assertions.assertFalse(stack.empty());

        stack.pop();

        Assertions.assertTrue(stack.empty());
    }
}
