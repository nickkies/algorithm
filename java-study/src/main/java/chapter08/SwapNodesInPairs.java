package chapter08;

public class SwapNodesInPairs {
    public static ListNode solution(ListNode head) {
        if (head == null || head.next == null) return head;

        ListNode newHead = head.next;
        head.next = solution(head.next.next);
        newHead.next = head;
        
        return newHead;
    }
}
