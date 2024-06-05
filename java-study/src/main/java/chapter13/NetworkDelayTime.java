package chapter13;

import java.util.*;

public class NetworkDelayTime {
    public static int solution(int[][] times, int n, int k) {
        Map<Integer, Integer> map = new HashMap<>();
        Map<Integer, List<int[]>> graph = new HashMap<>();
        PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]));

        for (int[] time : times) {
            graph.computeIfAbsent(time[0], x -> new ArrayList<>()).add(new int[]{time[1], time[2]});
        }

        pq.offer(new int[]{k, 0});

        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            int node = cur[0];
            int d = cur[1];

            if (map.containsKey(node)) continue;

            map.put(node, d);

            if (graph.containsKey(node)) {
                for (int[] edge : graph.get(node)) {
                    int next = edge[0];

                    if (!map.containsKey(next)) pq.offer(new int[]{next, edge[1] + d});
                }
            }
        }

        if (map.size() != n) return -1;

        return Collections.max(map.values());
    }
}
