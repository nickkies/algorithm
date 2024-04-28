package chapter11;

import java.util.HashMap;
import java.util.Map;

public class LongestSubstringWithoutRepeatingCharacters {
    public static int solution(String s) {
        Map<Character, Integer> map = new HashMap<>();
        int maxLen = 0;
        int start = 0;

        for (int i = 0; i < s.length(); i++) {
            char curr = s.charAt(i);

            if (map.containsKey(curr)) {
                start = Math.max(start, map.get(curr) + 1);
            }

            maxLen = Math.max(maxLen, i - start + 1);

            map.put(curr, i);
        }

        return maxLen;
    }
}
