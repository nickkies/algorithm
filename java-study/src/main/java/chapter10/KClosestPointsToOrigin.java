package chapter10;

import java.util.*;

public class KClosestPointsToOrigin {
    public static int[][] priorityQueueSolution(int[][] points, int k) {
        if (points == null || points.length == 0 || k <= 0) return null;

        PriorityQueue<Map<String, Object>> queue = new PriorityQueue<>(Comparator.comparingLong(m -> (long) m.get("distance")));

        for (int[] point : points) {
            Long distance = (long) point[0] * point[0] + (long) point[1] * point[1];
            queue.add(Map.of("point", point, "distance", distance));
        }

        int[][] res = new int[k][2];
        for (int i = 0; i < k; i++) {
            res[i] = (int[]) Objects.requireNonNull(queue.poll()).get("point");
        }

        return res;
    }

    public static int[][] streamSolution(int[][] points, int k) {
        return Arrays.stream(points)
                .sorted(Comparator.comparingLong(point -> (long) point[0] * point[0] + (long) point[1] * point[1]))
                .limit(k)
                .toArray(int[][]::new);
    }
}
