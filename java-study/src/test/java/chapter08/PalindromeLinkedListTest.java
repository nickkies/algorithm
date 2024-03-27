package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class PalindromeLinkedListTest {
    @Test
    public void test() {
        ListNode input = ListNode.of(1, 2, 3, 2, 1);
        
        Assertions.assertTrue(PalindromeLinkedList.solution(input));
    }
}
