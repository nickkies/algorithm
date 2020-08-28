# 퀵정렬(quick sort)와 합병정렬(merge sort)이 차이점을 서술 하시오.
#
# Describe differences between quick sort and merge sort.
def partition(arr, low, high):
    i = (low - 1)
    pivot = arr[high]
    for j in range(low, high):
        if arr[j] <= pivot:
            i = i + 1
            arr[i], arr[j] = arr[j], arr[i]

    arr[i + 1], arr[high] = arr[high], arr[i + 1]
    return (i + 1)


# Function to do Quick sort
def quickSort(arr, low, high):
    if low < high:
        pi = partition(arr, low, high)
        quickSort(arr, low, pi - 1)
        quickSort(arr, pi + 1, high)

# 시간 복잡도:
# 최고: O(nlogn)
# 평균: O(nlogn)
# 최악: O(n^2)
#
# 장점: 다른 자료구조를 만들지 않고 주어진 메모리 변환으로 정렬을 합니다.
# 단점: 최악의 경우 O(n^2)의 시간복잡도를 가집니다.



# Function to do Merge sort
def mergeSort(alist):
    if len(alist) > 1:
        mid = len(alist) // 2
        lefthalf = alist[:mid]
        righthalf = alist[mid:]

        mergeSort(lefthalf)
        mergeSort(righthalf)

        i = 0
        j = 0
        k = 0

        while i < len(lefthalf) and j < len(righthalf):
            if lefthalf[i] < righthalf[j]:
                alist[k] = lefthalf[i]
                i = i + 1
            else:
                alist[k] = righthalf[j]
                j = j + 1
            k = k + 1

        while i < len(lefthalf):
            alist[k] = lefthalf[i]
            i = i + 1
            k = k + 1

        while j < len(righthalf):
            alist[k] = righthalf[j]
            j = j + 1
            k = k + 1

# 시간 복잡도:
# 최고: O(nlogn)
# 평균: O(nlogn)
# 최악: O(nlogn)
#
# 장점: 시간복잡도가 항상 O(nlogn) 입니다.
# 단점: 병합을 하는 단계에서 새로운 자료구조를 이용함으로 O(n)의 새로운 메모리가 추가적으로 필요합니다.
