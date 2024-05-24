package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;

public class SubsetsTest {
    @Test
    public void test() {
        List<List<Integer>> answer = List.of(
                List.of(),
                List.of(1),
                List.of(2),
                List.of(4),
                List.of(1, 2),
                List.of(1, 4),
                List.of(2, 4),
                List.of(1, 2, 4)
        );
        List<List<Integer>> output = Subsets.solution(new int[]{1, 2, 4});
        
        Assertions.assertTrue(answer.containsAll(output) && output.containsAll(answer));
    }
}
