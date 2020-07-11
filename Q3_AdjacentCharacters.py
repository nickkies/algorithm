# 반복된 알파벳으로 이루어진 문자배열이 주어지면 연속으로 중복된 알파벳이 없도록 문자배열을 재배열하여 리턴하시오. 불가능 하다면 empty string을 리턴하시오.
# Given a string with repeated characters, rearrange the string so that no two adjacent characters are the same. If this is impossible, return an empty string.
#
# input: "aaabbc"
# output: "ababac"
#
# input: "aaac"
# output: ""
from collections import defaultdict
import heapq


def rearrange(string):
    frequencies = defaultdict(int)
    for letter in string:
        frequencies[letter] += 1

    heap = []
    for char, count in frequencies.items():
        heapq.heappush(heap, (-count, char))

    count, char = heapq.heappop(heap)
    result = [char]

    while heap:
        last = (count + 1, char)
        count, char = heapq.heappop(heap)
        result.append(char)

        if last[0] < 0:
            heapq.heappush(heap, last)

    if len(result) == len(string):
        return "".join(result)
    else:
        return ""


input1 = "aaabbc"
input2 = "aaac"

output1 = rearrange(input1)
output2 = rearrange(input2)

print(output1)
print(output2)

