package chapter09;

public class RemoveDuplicateLetters {
    public static String solution(String s) {
        if (s == null || s.isEmpty()) return "";

        StringBuilder sb = new StringBuilder();
        int[] counter = new int[26];
        boolean[] used = new boolean[26];
        char[] arr = s.toCharArray();

        for (char c : arr) counter[c - 'a']++;

        for (char c : s.toCharArray()) {
            counter[c - 'a']--;
            if (used[c - 'a']) continue;

            while (!sb.isEmpty() && c < sb.charAt(sb.length() - 1) && counter[sb.charAt(sb.length() - 1) - 'a'] > 0) {
                used[sb.charAt(sb.length() - 1) - 'a'] = false;
                sb.deleteCharAt(sb.length() - 1);
            }

            sb.append(c);
            used[c - 'a'] = true;
        }

        return sb.toString();
    }
}
