package chapter12;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class LetterCombinationsOfAPhoneNumber {
    private static void dfs(List<String> res, Map<Character, List<Character>> dict, String digits, int idx, StringBuilder path) {
        if (digits.length() == path.length()) {
            res.add(String.valueOf(path));
            return;
        }

        for (Character c : dict.get(digits.charAt(idx))) {
            dfs(res, dict, digits, idx + 1, new StringBuilder(path).append(c));
        }
    }


    public static List<String> solution(String digits) {
        if (digits.isEmpty()) return null;

        List<String> res = new ArrayList<>();
        Map<Character, List<Character>> dict = Map.of(
                '0', List.of(),
                '1', List.of(),
                '2', List.of('a', 'b', 'c'),
                '3', List.of('d', 'e', 'f'),
                '4', List.of('g', 'h', 'i'),
                '5', List.of('j', 'k', 'l'),
                '6', List.of('m', 'n', 'o'),
                '7', List.of('p', 'q', 'r', 's'),
                '8', List.of('t', 'u', 'v'),
                '9', List.of('w', 'x', 'y', 'z')
        );

        dfs(res, dict, digits, 0, new StringBuilder());

        return res;
    }
}
