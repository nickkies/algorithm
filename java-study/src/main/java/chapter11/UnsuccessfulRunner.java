package chapter11;

import java.util.HashMap;
import java.util.Map;

public class UnsuccessfulRunner {
    public static String solution(String[] participants, String[] completions) {
        Map<String, Integer> map = new HashMap<>();

        for (String participant : participants) {
            map.put(participant, map.getOrDefault(participant, 0) + 1);
        }

        for (String completer : completions) {
            int count = map.get(completer);

            if (count == 1)  map.remove(completer);
            else map.put(completer, count - 1);
        }

        return map.keySet().iterator().next();
    }
}
