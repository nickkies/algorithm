package chapter13;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class 게임_맵_최단_거리_test {
    @Test
    public void test() {
        int[][] input1 = {
            {1, 0, 1, 1, 1},
            {1, 0, 1, 0, 1},
            {1, 0, 1, 1, 1},
            {1, 1, 1, 0, 1},
            {0, 0, 0, 0, 1},
        };
        int[][] input2 = {
            {1, 0, 1, 1, 1},
            {1, 0, 1, 0, 1},
            {1, 0, 1, 1, 1},
            {1, 1, 1, 0, 0},
            {0, 0, 0, 0, 1},
        };

        Assertions.assertEquals(게임_맵_최단_거리.solution(input1), 11);
        Assertions.assertEquals(게임_맵_최단_거리.solution(input2), -1);
    }
}
