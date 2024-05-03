package chapter11;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class UnsuccessfulRunnerTest {
    @Test
    public void test() {
        String[] participant1 = { "레오", "키키", "읟엔"};
        String[] completion1 = {"읟엔", "키키"};
        String answer1 = "레오";
        String[] participant2 = {"뫄뤼놔", "조시파", "니꼴라", "븬코", "픠리퐈"};
        String[] completion2 = {"조시파","픠리퐈", "뫄뤼놔",  "니꼴라" };
        String answer2 = "븬코";

        Assertions.assertEquals(완주하지_못한_선수.solution(participant1, completion1), answer1);
        Assertions.assertEquals(완주하지_못한_선수.solution(participant2, completion2), answer2);

    }
}
