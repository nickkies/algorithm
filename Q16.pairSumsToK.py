# 정수 배열과 정수 K가 주어지면 원소 3개의 합으로 K가 만들어지는지 체크하시오.
# Given an integer array and an integer K, check if sum of 3 elements from the array equals to K.

# 방법 1 시간 복잡도: O(n^3)
def solution(arr, k):
    for x in range(len(arr)):
        for y in range(x + 1, len(arr)):
            for z in range(y + 1, len(arr)):
                if arr[x] + arr[y] + arr[z] == k:
                    return True
    return False


# 방법 2 시간복잡도: O(n log n) + O(n^2) = O(n^2)
def solution(arr, k):
    def pair_sums_to_k(arr, k, start):
        lo = start
        hi = len(arr) - 1
        while lo < hi:
            if arr[lo] + arr[hi] == k:
                return True
            elif arr[lo] + arr[hi] < k:
                lo += 1
            else:
                hi -= 1
        return False

    arr.sort()
    for i in range(len(arr) - 2):
        if pair_sums_to_k(arr, k - arr[i], i + 1):
            return True

    return False
