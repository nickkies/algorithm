package chapter10;

public class MyCircularDeque {
    private static class DoubleListNode {
        DoubleListNode prev;
        DoubleListNode next;
        int val;

        public DoubleListNode(int val) {
            this.val = val;
        }

        public DoubleListNode(int val, DoubleListNode prev, DoubleListNode next) {
            this.val = val;
            this.prev = prev;
            this.next = next;
        }
    }

    private final int len;
    private int size;
    private final DoubleListNode head;
    private final DoubleListNode tail;

    public MyCircularDeque(int k) {
        this.head = new DoubleListNode(-1);
        this.tail = new DoubleListNode(-1);
        this.head.next = tail;
        this.tail.prev = head;
        this.len = k;
        this.size = 0;
    }

    public boolean insertFront(int value) {
        if (isFull()) return false;
        DoubleListNode newNode = new DoubleListNode(value, head, head.next);
        head.next.prev = newNode;
        head.next = newNode;
        size++;
        return true;
    }

    public boolean insertLast(int value) {
        if (isFull()) return false;
        DoubleListNode newNode = new DoubleListNode(value, tail.prev, tail);
        tail.prev.next = newNode;
        tail.prev = newNode;
        size++;
        return true;
    }

    public boolean deleteFront() {
        if (isEmpty()) return false;
        DoubleListNode toDelete = head.next;
        head.next = toDelete.next;
        toDelete.next.prev = head;
        toDelete.next = null;
        toDelete.prev = null;
        size--;
        return true;
    }

    public boolean deleteLast() {
        if (isEmpty()) return false;
        DoubleListNode toDelete = tail.prev;
        tail.prev = toDelete.prev;
        toDelete.prev.next = tail;
        toDelete.next = null;
        toDelete.prev = null;
        size--;
        return true;
    }

    public int getFront() {
        if (isEmpty()) return -1;
        return head.next.val;
    }

    public int getRear() {
        if (isEmpty()) return -1;
        return tail.prev.val;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public boolean isFull() {
        return size == len;
    }
}
