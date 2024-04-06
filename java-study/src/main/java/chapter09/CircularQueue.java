package chapter09;

public class CircularQueue {
    private final int[] queue;

    private final int len;

    private int front;

    private int rear;

    private int size;

    public CircularQueue (int len) {
        this.queue = new int[len];
        this.len = len;
        this.front = 0;
        this.rear = -1;
        this.size = 0;
    }

    public boolean enQueue(int val) {
        if (isFull()) return false;

        rear = cal(rear);
        queue[rear] = val;
        size++;

        return true;
    }

    /**
     * CircularQueue에 요소를 강제로 삽입합니다.
     * 만약 큐가 가득 차있다면, 가장 앞에 있는 요소를 제거하고 해당 요소를 반환합니다.
     *
     * @param val 삽입할 요소의 값
     * @return 큐가 가득 차있어서 제거된 요소의 값, 큐가 가득 차있다면 -1 반환
     */
    public int enQueueForce(int val) {
        int res;

        if (isFull()) {
            res = front();
            front++;
        } else {
            res = -1;
            size++;
        }

        rear = cal(rear);
        queue[rear] = val;

        return res;
    }

    public boolean deQueue() {
        if (isEmpty()) return false;

        front = cal(front);
        size--;

        return true;
    }

    public int front() {
        return (isEmpty()) ? -1 : queue[front];
    }

    public int rear() {
        return (isEmpty()) ? -1 : queue[rear];
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public boolean isFull() {
        return len == size;
    }

    private int cal(int x) {
        return (x + 1) % len;
    }
}
