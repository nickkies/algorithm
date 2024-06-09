package chapter14;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class BalancedBinaryTreeTest {
    @Test
    public void test() {
        TreeNode<Integer> input1 = TreeNode.of(new Integer[]{1, 9, 20, null, null, 15, 7});
        TreeNode<Integer> input2 = TreeNode.of(new Integer[]{1, 2, 3, 4, 5, null, null, 6, 7});

        Assertions.assertTrue(BalancedBinaryTree.solution(input1));
        Assertions.assertFalse(BalancedBinaryTree.solution(input2));
    }
}
