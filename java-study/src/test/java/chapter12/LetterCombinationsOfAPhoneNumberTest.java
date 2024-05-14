package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.List;

public class LetterCombinationsOfAPhoneNumberTest {
    @Test
    public void test() {
        List<String> outputs = LetterCombinationsOfAPhoneNumber.solution("24");
        List<String> answer = List.of("ag", "ah", "ai", "bg", "bh", "bi", "cg", "ch", "ci");

        Assertions.assertLinesMatch(outputs, answer);
    }
}
