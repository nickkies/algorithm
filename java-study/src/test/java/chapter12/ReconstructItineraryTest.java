package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;

public class ReconstructItineraryTest {
    @Test
    public void test() {
        List<List<String>> input1 = List.of(
            List.of("MUC", "ICN"),
            List.of("JFK", "MUC"),
            List.of("SFO", "SJC"),
            List.of("ICN", "SFO")
        );
        List<String> answer1 = List.of("JFK", "MUC", "ICN", "SFO", "SJC");
        List<List<String>> input2 = List.of(
            List.of("JFK", "ICN"),
            List.of("JFK", "ATL"),
            List.of("ICN", "ATL"),
            List.of("ATL", "ICN"),
            List.of("ATL", "JFK")
        );
        List<String> answer2 = List.of("JFK", "ATL", "ICN", "ATL", "JFK", "ICN");

        Assertions.assertLinesMatch(ReconstructItinerary.solution(input1), answer1);
        Assertions.assertLinesMatch(ReconstructItinerary.solution(input2), answer2);
    }
}
