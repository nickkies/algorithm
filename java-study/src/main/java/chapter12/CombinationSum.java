package chapter12;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.List;

public class CombinationSum {
    private static void dfs(List<List<Integer>> res, int[] candidates, int start, int target, Deque<Integer> path) {
        if (target < 0) return;
        if (target == 0) {
            res.add(new ArrayList<>(path));
            return;
        }

        for (int i = start; i < candidates.length; i++) {
            path.addLast(candidates[i]);
            dfs(res, candidates, i, target - candidates[i], path);
            path.removeLast();
        }
    }

    public static List<List<Integer>> solution(int[] candidates, int target) {
        List<List<Integer>> res = new ArrayList<>();
        dfs(res, candidates, 0, target, new ArrayDeque<>());

        return res;
    }
}
