# 1~N 까지 있는 정수 배열에 원소 하나가 없어졌습니다. 없어진 원소의 값을 구하시오.
# Given an integer array of 1~N except one number, find the missing integer.

input = [1, 2, 3, 4, 5, 6, 8, 9, 10]


def findMissingNum(input):
    n = len(input) + 1
    total_sum = n * (n + 1) / 2
    array_sum = sum(input)
    return total_sum - array_sum


print(findMissingNum(input))
