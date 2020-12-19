# 이중 연결 리스트 (Doubly linked list)를 구현하시오.
#
# add(value): 주어진 value를 리스트 처음 노드로 등록.
# search(value): Value 를 가진 노드를 리턴.
# remove(node): 주어진 노드를 연결 리스트에서 제거.

class Node:
    def __init__(self, data):
        self.data = data
        self.next = None
        self.prev = None


class LinkedList:
    def __init__(self):
        self.head = None

    def add(self, data):
        node = Node(data)
        if self.head is None:
            self.head = node
        else:
            node.next = self.head
            node.next.prev = node
            self.head = node

    def search(self, k):
        p = self.head
        if p != None:
            while p.next is not None:
                if p.data == k:
                    return p
                p = p.next
            if p.data == k:
                return p
        return None

    def remove(self, p):
        tmp = p.prev
        p.prev.next = p.next
        p.prev = tmp
