# 단방향 연결 리스트(singly linked list)가 주어지면 총 합이 0으로 되는 연결된 노드들을 뺀 뒤 남은 노드의 값을 프린트 하시오.
# Given a linked list, remove consecutive nodes that sum to zero. Print the values of leftover nodes.
#
# input: 3 -> (-5) -> 5 -> 1 -> 2 -> 3
# output: 3 -> 1 -> 2 -> 3
#
# input: 1 -> 2 -> 3 -> 4 -> (-10) -> 5
# output: 5
#
# input: 10 -> (-3) -> (-4) -> (-3) -> 1
# output: 1


class LinkedList:
    def __init__(self, data, next=None):
        self.data = data
        self.next = next


def solution(head):
    start = end = head

    while start:
        end = start
        total = 0
        skip = False

        while end:
            total += end.data
            if total == 0:
                start = end
                skip = True
                break
            end = end.nextK

        if not skip:
            print(start.data)

        start = start.next
