package chapter10;

import java.util.PriorityQueue;

public class 더_맵게 {
    public static int solution(int[] scovilles, int k) {
        PriorityQueue<Integer> queue = new PriorityQueue<>(scovilles.length);

        for (int scoville : scovilles) queue.add(scoville);

        int res = 0;
        while (queue.size() > 1) {
            queue.add(queue.poll() + (queue.poll() * 2));

            res++;

            if (queue.peek() > k) return res;
        }

        return queue.peek() < k ? -1 : res;
    }
}
