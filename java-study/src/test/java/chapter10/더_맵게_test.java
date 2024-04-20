package chapter10;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class 더_맵게_test {
    @Test
    public void test() {
        Assertions.assertEquals(더_맵게.solution(new int[]{1, 2, 3, 9, 10, 11}, 7), 2);
    }
}
