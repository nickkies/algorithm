package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class 여행경로_test {
    @Test
    public void test() {
        String[][] input1 = {
            {"ICN", "JFK"},
            {"HND", "IAD"},
            {"JFK", "HND"}
        };
        String[] answer1 = {"ICN", "JFK", "HND", "IAD"};
        String[][] input2 = {
            {"ICN", "SFO"},
            {"ICN", "ATL"},
            {"SFO", "ATL"},
            {"ATL", "ICN"},
            {"ATL", "SFO"}
        };
        String[] answer2 = {"ICN", "ATL", "ICN", "SFO", "ATL", "SFO"};

        Assertions.assertArrayEquals(여행경로.solution(input1), answer1);
        Assertions.assertArrayEquals(여행경로.solution(input2), answer2);
    }
}
