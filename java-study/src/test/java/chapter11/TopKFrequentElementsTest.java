package chapter11;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class TopKFrequentElementsTest {
    @Test
    public void test() {
        Assertions.assertArrayEquals(TopKFrequentElements.solution(new int[]{1, 1, 1 ,2 ,2 ,2, 3, 4}, 2), new int[]{1, 2});
    }
}
