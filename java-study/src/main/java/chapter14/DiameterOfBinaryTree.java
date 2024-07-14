package chapter14;

public class DiameterOfBinaryTree {
    private static int longest = 0;

    private static int dfs(TreeNode<Integer> node) {
        if (node == null) return 0;

        int left = dfs(node.left);
        int right = dfs(node.right);

        longest = Math.max(longest, left + right + 2);

        return Math.max(left, right) + 1;
    }

    public static int solution(TreeNode<Integer> root) {
        dfs(root);
        return longest;
    }
}
