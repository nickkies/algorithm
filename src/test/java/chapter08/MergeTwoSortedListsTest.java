package chapter08;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class MergeTwoSortedListsTest {
    @Test
    public void test() {
        ListNode list1 = ListNode.of(1, 2, 5);
        ListNode list2 = ListNode.of(1, 3, 4);
        ListNode answer = ListNode.of(1, 1, 2, 3, 4, 5);

        Assertions.assertEquals(answer, MergeTwoSortedLists.solution(list1, list2));

    }

}
