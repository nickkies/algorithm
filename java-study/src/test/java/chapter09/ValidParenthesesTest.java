package chapter09;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class ValidParenthesesTest {

    @Test
    public void failTest() {
        String input = "(]";
        Assertions.assertFalse(ValidParentheses.solution(input));
    }

    @Test
    public void successTest() {
        String input1 = "()";
        String input2 = "()[]{}";

        Assertions.assertTrue(ValidParentheses.solution(input1));
        Assertions.assertTrue(ValidParentheses.solution(input2));
    }
}
