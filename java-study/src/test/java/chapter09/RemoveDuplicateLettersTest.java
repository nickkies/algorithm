package chapter09;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class RemoveDuplicateLettersTest {
    @Test
    public void test() {
        Assertions.assertEquals(RemoveDuplicateLetters.solution("bcabc"), "abc");
        Assertions.assertEquals(RemoveDuplicateLetters.solution("cbacdcbc"), "acdb");
    }
}
