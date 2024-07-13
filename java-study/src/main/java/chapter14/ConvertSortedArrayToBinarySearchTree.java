package chapter14;

public class ConvertSortedArrayToBinarySearchTree {
    private static TreeNode<Integer> recursive(int[] nums, int low, int high) {
        if (low > high) return null;

        int mid = low + (high - low) / 2;

        TreeNode<Integer> root = new TreeNode<>(nums[mid]);
        root.left = recursive(nums, low, mid - 1);
        root.right = recursive(nums, mid + 1, high);

        return root;
    }

    public static TreeNode<Integer> solution(int[] nums) {
        if (nums == null || nums.length == 0) return null;
        return recursive(nums, 0, nums.length - 1);
    }
}
