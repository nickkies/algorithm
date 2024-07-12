package chapter14;

public class BinarySearchTreeToGreaterSumTree {
    private static int val;

    public static TreeNode<Integer> solution(TreeNode<Integer> root) {
        if (root == null) return null;

        solution(root.right);

        val += root.val;
        
        root.val = val;

        solution(root.left);

        return root;
    }
}
