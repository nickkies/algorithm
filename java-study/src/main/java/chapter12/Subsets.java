package chapter12;

import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Deque;
import java.util.List;

public class Subsets {
    public static void dfs(List<List<Integer>> res, int[] nums, int start, Deque<Integer> path) {
        res.add(new ArrayList<>(path));

        for (int i = start; i < nums.length; i++) {
            path.addLast(nums[i]);
            dfs(res, nums, i + 1, path);
            path.removeLast();
        }
    }

    public static List<List<Integer>> solution(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();

        dfs(res, nums, 0, new ArrayDeque<>());

        return res;
    }
}
