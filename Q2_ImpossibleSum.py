# 정렬된 양수(positive integer) 배열이 주어지면, 배열 원소들의 합으로 만들수 없는 가장 작은 양수를 구하시오.
# 단, 시간복잡도는 O(n) 이여야 합니다.
# Given an array of positive integers, find the smallest positive integer that cannot be created by adding elements in the array.
#
# input: [1, 2, 3, 8]
# output: 7

input = [1, 2, 3, 8]


def solution(nums):
    smallest_impossible = 1
    for n in nums:
        if n <= smallest_impossible:
            smallest_impossible += n
        else:
            return smallest_impossible


print(solution(input))