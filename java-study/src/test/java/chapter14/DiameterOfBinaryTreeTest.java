package chapter14;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class DiameterOfBinaryTreeTest {
    @Test
    public void test() {
        TreeNode<Integer> input = TreeNode.of(new Integer[]{1, 2, 6, 4, 5});
        Assertions.assertEquals(DiameterOfBinaryTree.solution(input), 3);
    }
}
