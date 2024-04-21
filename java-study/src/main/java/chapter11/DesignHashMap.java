package chapter11;

import chapter08.ListNode;

public class DesignHashMap {
    private static class Node {
        int key;
        int val;
        Node next;

        Node(int key, int val) {
            this.key = key;
            this.val = val;
        }
    }

    final Node[] nodes = new Node[1000000];

    public void put(int key, int val) {
        int idx = hash(key);
        Node head = nodes[idx];

        for (Node node = head; node != null; node = node.next) {
            if (node.key == key) {
                node.val = val;
                return;
            }
        }

        Node node = new Node(key, val);
        node.next = head;
        nodes[idx] = node;
    }

    public int get(int key) {
        int idx = hash(key);
        Node node = nodes[idx];

        while (node != null) {
            if (node.key == key) return node.val;
            node = node.next;
        }

        return -1;
    }

    public void remove(int key) {
        int idx = hash(key);
        Node head = nodes[idx];
        Node prev = null;

        for (Node node = head; node != null; node = node.next) {
            if (node.key == key) {
                if (prev == null) nodes[idx] = node.next;
                else prev.next = node.next;
                return;
            }
            prev = node;
        }
    }

    private int hash(int key) {
        return key % nodes.length;
    }
}
