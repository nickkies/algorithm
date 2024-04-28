package chapter11;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class LongestSubstringWithoutRepeatingCharactersTest {
    @Test
    public void test() {
        Assertions.assertEquals(LongestSubstringWithoutRepeatingCharacters.solution("abcabcbbc"), 3);
    }
}
