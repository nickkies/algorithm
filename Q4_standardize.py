# 주어진 정수를 2진법으로 나타내었을때 1의 갯수를 리턴하시오.
# Given an integer, count number of 1s in binary representation of an integer.
# 시간 복잡도: O(log n)
#
# input: 6 // 110
# output: 2
#
# input: 13 // 1101
# output: 3


def countsetbits(n):
    count = 0
    while (n):
        n &= ( n - 1 )
        count += 1
    return count


input1 = 6
input2 = 13

output1 = countsetbits(input1)
output2 = countsetbits(input2)

print(output1)
print(output2)