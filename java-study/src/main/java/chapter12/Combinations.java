package chapter12;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Combinations {
    private static void dfs(List<List<Integer>> res, LinkedList<Integer> el, int n, int start, int k) {
        if (k == 0) {
            res.add(new ArrayList<>(el));
            return;
        }

        for (int i = start; i <= n; i++) {
            el.add(i);
            dfs(res, el, n, i + 1, k - 1);
            el.removeLast();
        }
    }

    public static List<List<Integer>> solution(int n, int k) {
        List<List<Integer>> res = new ArrayList<>();

        dfs(res, new LinkedList<>(), n, 1, k);

        return res;
    }
}
