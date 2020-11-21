# 0, 1, 2로 이루어진 배열을 가장 효율적으로 정렬 하시오. 시간복잡도 O(n).
# Given an array consisting of 0, 1 and 2s, sort this array.

# Input: [0, 1, 2, 2, 0, 0, 0, 1]
# Output: [0, 0, 0, 0, 1, 1, 2, 2]


def sort(a):
    low = 0
    high = len(a) - 1
    i = 0
    while i <= high:
        if a[i] == 0:
            a[low], a[i] = a[i], a[low]
            low = low + 1
            i = i + 1
        elif a[i] == 1:
            i = i + 1
        else:
            a[i], a[high] = a[high], a[i]
            high = high - 1
    return a


print(sort([0, 1, 2, 2, 0, 0, 0, 1]))