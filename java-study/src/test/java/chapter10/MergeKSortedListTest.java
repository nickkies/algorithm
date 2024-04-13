package chapter10;

import chapter08.ListNode;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class MergeKSortedListTest {
    @Test
    public void test() {
        ListNode[] inputs = {
                ListNode.of(1, 4, 5),
                ListNode.of(1, 3, 4),
                ListNode.of(2, 7),
        };
        ListNode answer = ListNode.of(1, 1, 2, 3, 4, 4, 5, 7);

        Assertions.assertEquals(MergeKSortedList.solution(inputs), answer);
    }
}
