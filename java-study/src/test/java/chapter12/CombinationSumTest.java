package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class CombinationSumTest {
    @Test
    public void test() {
        Set<List<Integer>> answer = Set.of(
                List.of(2, 2, 2, 2),
                List.of(2, 3, 3),
                List.of(3, 5),
                List.of(8)
        );

        Set<List<Integer>> output = new HashSet<>(CombinationSum.solution(new int[]{2, 3, 5, 8}, 8));
        Assertions.assertTrue(answer.containsAll(output) && output.containsAll(answer));
    }
}
