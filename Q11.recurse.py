# 이진탐색트리안에 X보다 크고 Y보다 작은 모든 노드 값을 프린트 하시오.
# Given a binary search tree, print all node values that are bigger than X and smaller than Y.

def Recurse(node, x, y):
    if node is None:
        return

    if x < node.data:
        Recurse(node.left, x, y)

    if x <= node.data <= y:
        print
        node.data,

    if y > node.data:
        Recurse(node.right, x, y)
