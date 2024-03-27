package chapter08;

public class PalindromeLinkedList {
    public static boolean solution(ListNode head) {

        // 중간 지점 찾기
        ListNode middle = findMiddle(head);

        // 중간부터 끝까지 리스트를 역순으로 만들기
        ListNode reversedSecondHalf = reverseList(middle);

        // 리스트 비교
        return compareLists(head, reversedSecondHalf);
    }

    private static ListNode findMiddle(ListNode head) {
        ListNode fast = head, slow = head;

        while (fast != null && fast.next != null) {
            fast = fast.next.next;
            slow = slow.next;
        }

        // 홀수는 중앙 생략
        if (fast != null) slow = slow.next;

        return slow;
    }

    private static ListNode reverseList(ListNode head) {
        ListNode prev = null;

        while (head != null) {
            ListNode next = head.next;
            head.next = prev;
            prev = head;
            head = next;
        }

        return prev;
    }

    private static boolean compareLists(ListNode head1, ListNode head2) {
        while (head2 != null) {
            if (head1.val != head2.val) return false;

            head1 = head1.next;
            head2 = head2.next;
        }

        return true;
    }
}
