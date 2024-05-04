package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class CombinationsTest {
    @Test
    public void test() {
        Set<List<Integer>> answer = Set.of(
                List.of(1, 2, 3),
                List.of(1, 2, 4),
                List.of(1, 3, 4),
                List.of(2, 3, 4),
                List.of(1, 2, 5),
                List.of(1, 3, 5),
                List.of(1, 4, 5),
                List.of(2, 3, 5),
                List.of(2, 4, 5),
                List.of(3, 4, 5)
        );
        Set<List<Integer>> output = new HashSet<>(Combinations.solution(5, 3));

        Assertions.assertTrue(answer.containsAll(output) && output.containsAll(answer));
    }
}
