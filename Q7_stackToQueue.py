# 스택(Stack)을 이용해서 큐(Queue)를 구현하시오.
#
# Implement a queue using stacks.


class MyQueue(object):
    def __init__(self):
        self.first = []
        self.second = []

    def peek(self):
        if not self.second:
            while self.first:
                self.second.append(self.first.pop())
        return self.second[len(self.second)-1]

    def pop(self):
        if not self.second:
            while self.first:
                self.second.append(self.first.pop())
        return self.second.pop()

    def put(self, value):
        self.first.append(value)