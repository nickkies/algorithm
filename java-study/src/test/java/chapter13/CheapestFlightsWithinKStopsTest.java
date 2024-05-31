package chapter13;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class CheapestFlightsWithinKStopsTest {
    @Test
    public void test() {
        int[][] input = {
            {0, 1, 100},
            {1, 2, 200},
            {0, 2, 500}
        };

        Assertions.assertEquals(CheapestFlightsWithinKStops.solution(input, 0, 2, 0), 500);
    }
}
