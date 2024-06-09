package chapter14;

public class BalancedBinaryTree {
    private static int dfs(TreeNode<Integer> node) {
        if (node == null) return 0;

        int left = dfs(node.left);
        if (left == -1) return -1;

        int right = dfs(node.right);
        if (right == -1) return -1;

        if (Math.abs(left - right) > 1) return -1;

        return Math.max(left, right) + 1;
    }

    public static boolean solution(TreeNode<Integer> root) {
        return dfs(root) != -1;
    }
}
