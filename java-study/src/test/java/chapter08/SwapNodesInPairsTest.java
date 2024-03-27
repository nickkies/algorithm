package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class SwapNodesInPairsTest {
    @Test
    public void test() {
        ListNode input = ListNode.of(1, 2, 3, 4, 5, 6);
        ListNode answer = ListNode.of(2, 1, 4, 3, 6, 5);

        Assertions.assertEquals(answer, SwapNodesInPairs.solution(input));
    }
}
