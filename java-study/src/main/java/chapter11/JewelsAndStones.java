package chapter11;

import java.util.Set;
import java.util.stream.Collectors;

public class JewelsAndStones {
    public static int solution(String J, String S) {
        Set<Character> set = J.chars()
                .mapToObj(e -> (char) e)
                .collect(Collectors.toSet());

        return (int) S.chars()
                .mapToObj(e -> (char) e)
                .filter(set::contains)
                .count();
    }
}