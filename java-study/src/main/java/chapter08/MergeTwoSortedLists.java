package chapter08;

public class MergeTwoSortedLists {
    public static ListNode solution(ListNode list1, ListNode list2) {
        ListNode tmp = new ListNode(Integer.MIN_VALUE);
        ListNode cur = tmp;

        while (list1 != null && list2 != null) {
            if (list1.val < list2.val) {
                cur.next = list1;
                list1 = list1.next;
            } else {
                cur.next = list2;
                list2 = list2.next;
            }

            cur = cur.next;
        }

        cur.next = (list1 != null) ? list1 : list2;

        return tmp.next;
    }
}
