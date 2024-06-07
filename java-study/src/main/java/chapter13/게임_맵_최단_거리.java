package chapter13;

import java.util.*;

public class 게임_맵_최단_거리 {
    private static Queue<Position> pq;

    public static int solution(int[][] maps) {
        pq = new PriorityQueue<>(Comparator.comparingInt(o -> o.distance));
        Map<Integer, Position> dist = new LinkedHashMap<>();

        pq.add(new Position(0, 0, 1));

        while (!pq.isEmpty()) {
            Position cur = pq.poll();

            if (!dist.containsKey(cur.y * 1000 + cur.x)) {
                dist.put(cur.y * 1000 + cur.x, cur);
                findPath(cur.y, cur.x + 1, cur.distance, maps);
                findPath(cur.y, cur.x - 1, cur.distance, maps);
                findPath(cur.y + 1, cur.x, cur.distance, maps);
                findPath(cur.y - 1, cur.x, cur.distance, maps);
            }
        }

        int destinationKey = (maps.length - 1) * 1000 + maps[0].length - 1;

        if (dist.containsKey(destinationKey)) {
            return dist.get(destinationKey).distance;
        }

        return -1;
    }

    public static void findPath(int y, int x, int distance, int[][] maps) {
        if (y >= 0 && y < maps.length && x >= 0 && x < maps[0].length && maps[y][x] == 1) {
            maps[y][x] = 0;
            pq.add(new Position(y, x, distance + 1));
        }
    }

    record Position(int y, int x, int distance) {
    }
}
