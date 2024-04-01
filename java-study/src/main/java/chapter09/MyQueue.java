package chapter09;

import java.util.Stack;

public class MyQueue<T> {
    private final Stack<T> enqueue;
    private final Stack<T> dequeue;

    public MyQueue() {
        this.enqueue = new Stack<>();
        this.dequeue = new Stack<>();
    }

    public void push(T x) {
        enqueue.push(x);
    }

    public T pop() {
        if (dequeue.isEmpty()) {
            while (!enqueue.isEmpty()) {
                dequeue.push(enqueue.pop());
            }
        }
        return dequeue.pop();
    }

    public T peek() {
        if (dequeue.isEmpty()) {
            while (!enqueue.isEmpty()) {
                dequeue.push(enqueue.pop());
            }
        }
        return dequeue.peek();
    }

    public boolean empty() {
        return enqueue.isEmpty() && dequeue.isEmpty();
    }
}