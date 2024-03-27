package chapter08;

public class ListNode {
    int val;
    ListNode next;

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }

    public static ListNode of(int... values) {
        ListNode head = null;
        ListNode tail = null;

        for (int val : values) {
            ListNode newNode = new ListNode(val);

            if (head == null) {
                head = newNode;
            } else {
                tail.next = newNode;
            }

            tail = newNode;
        }

        return head;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ListNode list2)) return false;

        ListNode list1 = this;

        while (list1 != null && list2 != null) {
            if (list1.val != list2.val) return false;

            list1 = list1.next;
            list2 = list2.next;
        }

        return list1 == null && list2 == null;
    }
}
