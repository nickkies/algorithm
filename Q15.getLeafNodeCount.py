# 이진트리안에 모든 단말노드(leaf node)의 갯수를 구하시오.
# 트리의 루트노드가 주어집니다.
#
# Given a root node of a binary tree, count all leaf nodes.

class Node:
    def __init__(self, data):
        self.data = data
        self.left = None
        self.right = None


def getLeafNodeCount(node):
    if node is None:
        return 0
    if node.left is None and node.right is None:
        return 1
    else:
        return getLeafNodeCount(node.left) + getLeafNodeCount(node.right)
