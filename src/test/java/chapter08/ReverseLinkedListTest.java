package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class ReverseLinkedListTest {
    @Test
    public void test() {
        ListNode input = ListNode.of(1, 2, 3, 4, 5, 6);
        ListNode answer = ListNode.of(6, 5, 4, 3, 2, 1);
        
        Assertions.assertEquals(answer, ReverseLinkedList.solution(input));
    }
}
