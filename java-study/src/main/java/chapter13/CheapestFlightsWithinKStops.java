package chapter13;

import java.util.Comparator;
import java.util.HashMap;
import java.util.Map;
import java.util.PriorityQueue;

public class CheapestFlightsWithinKStops {
    public static int solution(int[][] flights, int src, int dst, int k) {
        Map<Integer, Map<Integer, Integer>> graph = new HashMap<>();
        for (int[] flight : flights) {
            graph.putIfAbsent(flight[0], new HashMap<>());
            graph.get(flight[0]).put(flight[1], flight[2]);
        }

        PriorityQueue<int[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> a[1]));
        pq.offer(new int[]{src, 0, 0});

        Map<Integer, Integer> best = new HashMap<>();

        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            int node = cur[0];
            int cost = cur[1];
            int stops = cur[2];

            if (node == dst) return cost;

            if (stops <= k) {
                Map<Integer, Integer> adj = graph.getOrDefault(node, new HashMap<>());
                for (Map.Entry<Integer, Integer> entry : adj.entrySet()) {
                    int nextNode = entry.getKey();
                    int nextCost = cost + entry.getValue();
                    int nextStops = stops + 1;

                    if (!best.containsKey(nextNode) || nextCost < best.get(nextNode)) {
                        pq.offer(new int[]{nextNode, nextCost, nextStops});
                        best.put(nextNode, nextCost);
                    }
                }
            }
        }

        return -1;
    }
}
