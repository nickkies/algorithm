package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class AddTwoNumbersTest {
    @Test
    public void test() {
        ListNode list1 = ListNode.of(2, 4, 3);
        ListNode list2 = ListNode.of(5, 6, 2);
        ListNode answer = ListNode.of(7, 0, 6);

        Assertions.assertEquals(answer, AddTwoNumbers.solution(list1, list2));
    }
}
