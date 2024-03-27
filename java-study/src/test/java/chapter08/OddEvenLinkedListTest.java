package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class OddEvenLinkedListTest {
    @Test
    public void test() {
        ListNode input = ListNode.of(1, 2, 3, 4, 5, 6);
        ListNode answer = ListNode.of(1, 3, 5, 2, 4, 6);

        Assertions.assertEquals(answer, OddEvenLinkedList.solution(input));
    }
}
