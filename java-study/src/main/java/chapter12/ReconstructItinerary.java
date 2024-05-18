package chapter12;

import java.util.*;

public class ReconstructItinerary {
    private static void dfs(List<String> res, Map<String, PriorityQueue<String>> fromToMap, String from) {
        while (fromToMap.containsKey(from) && !fromToMap.get(from).isEmpty()) {
            dfs(res, fromToMap, fromToMap.get(from).poll());
        }

        res.add(0, from);
    }

    public static List<String> solution(List<List<String>> tickets) {
        List<String> res = new ArrayList<>();
        Map<String, PriorityQueue<String>> formToMap = new HashMap<>();

        for (List<String> ticket : tickets) {
            formToMap.computeIfAbsent(ticket.get(0), k -> new PriorityQueue<>()).add(ticket.get(1));
        }

        dfs(res, formToMap, "JFK");

        return res;
    }
}
