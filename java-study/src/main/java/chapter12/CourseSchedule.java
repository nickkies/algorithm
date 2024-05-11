package chapter12;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class CourseSchedule {
    private static boolean dfs(Map<Integer, List<Integer>> finToTakeMap, Integer fin, List<Integer> takes, List<Integer> taken) {
        if (takes.contains(fin)) return true;
        if (taken.contains(fin)) return false;

        if (finToTakeMap.containsKey(fin)) {
            takes.add(fin);

            for (Integer take : finToTakeMap.get(fin)) {
                if (dfs(finToTakeMap, take, takes, taken)) return true;
            }

            takes.remove(fin);
            taken.add(fin);
        }

        return false;
    }

    public static boolean solution(int cnt, int[][] lists) {
        Map<Integer, List<Integer>> finToTakeMap = new HashMap<>(cnt);

        for (int[] list : lists) {
            finToTakeMap.computeIfAbsent(list[0], k -> new ArrayList<>()).add(list[1]);
        }

        List<Integer> takes = new ArrayList<>(cnt);
        List<Integer> taken = new ArrayList<>(cnt);

        for (Integer fin : finToTakeMap.keySet()) {
            if (dfs(finToTakeMap, fin, takes, taken)) return false;
        }

        return true;
    }
}
