package chapter09;

import java.util.ArrayDeque;
import java.util.Deque;

public class DailyTemperatures {
    public static int[] solution(int[] temperatures) {
        int[] res = new int[temperatures.length];
        Deque<Integer> stack = new ArrayDeque<>();

        for (int i = 0; i < temperatures.length; i++) {
            while (!stack.isEmpty() && temperatures[i] > temperatures[stack.peek()]) {
                int last = stack.pop();
                res[last] = i - last;
            }

            stack.push(i);
        }

        return res;
    }
}
