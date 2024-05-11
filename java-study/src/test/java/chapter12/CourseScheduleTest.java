package chapter12;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class CourseScheduleTest {
    @Test
    public void test() {
        int[][] input1 = {{1, 0}, {2, 1}};
        int[][] input2 = {{2, 1}, {1, 2}};

        Assertions.assertTrue(CourseSchedule.solution(3, input1));
        Assertions.assertFalse(CourseSchedule.solution(2, input2));
    }
}
