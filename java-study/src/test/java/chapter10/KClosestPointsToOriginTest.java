package chapter10;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.Random;

public class KClosestPointsToOriginTest {
    @Test
    public void test() {
        int MAX = 100000;
        int K = 1;

        int[][] points = new int[MAX][2];
        Random random = new Random();

        for (int i = 0; i < MAX; i++) {
            points[i][0] = random.nextInt(MAX * 2 + 1) - MAX;
            points[i][1] = random.nextInt(MAX * 2 + 1) - MAX;
        }

        int[][] answer = KClosestPointsToOrigin.priorityQueueSolution(points, K);

        long s1 = System.nanoTime();
        Assertions.assertArrayEquals(KClosestPointsToOrigin.priorityQueueSolution(points, K), answer);
        long e1 = System.nanoTime();
        long d1 = e1 - s1;

        long s2 = System.nanoTime();
        Assertions.assertArrayEquals(KClosestPointsToOrigin.streamSolution(points, K), answer);
        long e2 = System.nanoTime();
        long d2 = e2 - s2;

        long dif = d2 - d1;

        System.out.println("=======================================================================================");
        System.out.println("dif " + dif + " ns");
        System.out.println("=======================================================================================");

        Assertions.assertTrue(dif > 0);
    }
}
