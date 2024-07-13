package chapter14;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class ConvertSortedArrayToBinarySearchTreeTest {
    @Test
    public void test() {
        int[] input = {-10, -7, -3, 0, 5, 7, 9};
        TreeNode<Integer> answer = TreeNode.of(new Integer[]{0, -7, 7, -10, -3, 5, 9});
        TreeNode<Integer> output = ConvertSortedArrayToBinarySearchTree.solution(input);

        Assertions.assertEquals(answer, output);
    }
}
