package chapter09;

import java.util.LinkedList;
import java.util.Queue;

public class MyStack<T> {
    private final Queue<T> queue;
    private T topEl;

    public MyStack() {
        this.queue = new LinkedList<>();
    }

    public void push(T topEl) {
        queue.add(topEl);
        this.topEl = topEl;
    }

    public T pop() {
        for (int i = 0; i < queue.size() - 1; i++) {
            topEl = queue.remove();
            queue.add(topEl);
        }

        return queue.remove();
    }

    public T top() {
        return topEl;
    }

    public boolean empty() {
        return queue.isEmpty();
    }
}
