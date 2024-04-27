package chapter11;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class JewelsAndStonesTest {
    @Test
    public void test() {
        String J = "aA";
        String S = "aAAbbbb";

        Assertions.assertEquals(JewelsAndStones.solution(J, S), 3);
    }
}
