package chapter08;

public class AddTwoNumbers {
    public static ListNode solution(ListNode list1, ListNode list2) {
        ListNode node = new ListNode(0);
        ListNode root = node;
        int sum, carry = 0;

        while (list1 != null || list2 != null || carry != 0) {
            sum = carry;

            if (list1 != null) {
                sum += list1.val;
                list1 = list1.next;
            }

            if (list2 != null) {
                sum += list2.val;
                list2 = list2.next;
            }

            root.next = new ListNode(sum % 10);
            carry = sum / 10;
            root = root.next;
        }

        return node.next;
    }
}
