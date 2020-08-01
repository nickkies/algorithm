# 백만개의 정수가 들어있는 배열을 가장 빨리 정렬하시오. 모든 정수는 1조보다 작습니다.
# Sort an array with million integers.
def countingSort(arr, exp1):
    n = len(arr)
    output = [0] * (n)
    count = [0] * (10)

    for i in range(0, n):
        index = (arr[i] / exp1)
        count[(index) % 10] += 1

    for i in range(1, 10):
        count[i] += count[i - 1]

    i = n - 1
    while i >= 0:
        index = (arr[i] / exp1)
        output[count[(index) % 10] - 1] = arr[i]
        count[(index) % 10] -= 1
        i -= 1

    i = 0
    for i in range(0, len(arr)):
        arr[i] = output[i]


# Method to do Radix Sort
def radixSort(arr):
    max1 = max(arr)

    exp = 1
    while max1 / exp > 0:
        countingSort(arr, exp)
        exp *= 10