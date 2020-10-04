# 주어진 정수가 4의 거듭제곱인지 확인하시오.
# Given an integer, check if it is a power of 4.

def isPowerOfFour(n):
    if n == 0:
        return False
    while n != 1:
            if n % 4 != 0:
                return False
            n = n // 4
    return True


def isPowerOfFour2(n):
    count = 0
    # 조건 1: 1의 갯수가 하나인지 체크.
    if n and (not (n & (n - 1))):

        # 조건 2: 1 뒤에 0의 갯수를 셉니다.
        while n > 1:
            n >>= 1
            count += 1

        # 조건 2: 0의 갯수가 짝수여야합니다.
        if count % 2 == 0:
            return True
        else:
            return False