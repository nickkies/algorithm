package chapter14;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class BinarySearchTreeToGreaterSumTreeTest {
    @Test
    public void test() {
        TreeNode<Integer> input = TreeNode.of(new Integer[]{4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 9});
        TreeNode<Integer> answer = TreeNode.of(new Integer[]{31, 37, 22, 37, 36, 27, 16, null, null, null, 34, null, null, null, 9});
        Assertions.assertEquals(BinarySearchTreeToGreaterSumTree.solution(input), answer);
    }
}
