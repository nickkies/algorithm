package chapter12;

import java.util.*;

public class 여행경로 {
    public static String[] solution(String[][] tickets) {
        Map<String, PriorityQueue<String>> fromToMap = new HashMap<>();

        for (String[] ticket : tickets) {
            fromToMap.computeIfAbsent(ticket[0], k -> new PriorityQueue<>()).add(ticket[1]);
        }

        List<String> res = new ArrayList<>();
        Deque<String> stack = new ArrayDeque<>();

        stack.push("ICN");

        while (!stack.isEmpty()) {
            while (fromToMap.containsKey(stack.getFirst()) && !fromToMap.get(stack.getFirst()).isEmpty()) {
                stack.push(fromToMap.get(stack.getFirst()).poll());
            }

            res.add(0, stack.pop());
        }

        return res.toArray(new String[0]);
    }
}
