package chapter10;

import chapter08.ListNode;

import java.util.Comparator;
import java.util.PriorityQueue;

public class MergeKSortedList {
    public static ListNode solution(ListNode[] lists) {
        if (lists == null || lists.length == 0) return null;

        PriorityQueue<ListNode> queue = new PriorityQueue<>(Comparator.comparingInt(l -> l.val));

        ListNode dummy = new ListNode(0);
        ListNode tail = dummy;

        for (ListNode node : lists) {
            if (node != null) queue.add(node);
        }

        while (!queue.isEmpty()) {
            tail.next = queue.poll();
            tail = tail.next;

            if (tail.next != null) queue.add(tail.next);
        }

        return dummy.next;
    }
}
