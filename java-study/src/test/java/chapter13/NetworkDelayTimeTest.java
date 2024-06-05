package chapter13;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class NetworkDelayTimeTest {
    @Test
    public void test() {
        int[][] input = {
            {3, 1, 5},
            {3, 2, 2},
            {2, 1, 2},
            {3, 4, 1},
            {4, 5, 1},
            {5, 6, 1},
            {6, 7, 1},
            {7, 8, 1},
            {8, 1, 1},
        };
        Assertions.assertEquals(NetworkDelayTime.solution(input, 8, 3), 5);
    }
}
