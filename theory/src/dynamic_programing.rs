#[cfg(test)]
mod tests {
    fn fibo_recursive(n: usize) -> usize {
        if n <= 1 {
            n
        } else {
            fibo_recursive(n - 1) + fibo_recursive(n - 2)
        }
    }

    fn fibo_dynamic(n: usize, memo: &mut Vec<Option<usize>>) -> usize {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else if let Some(val) = memo[n] {
            return val;
        }

        let res = fibo_dynamic(n - 1, memo) + fibo_dynamic(n - 2, memo);
        memo[n] = Some(res);

        res
    }

    #[test]
    fn test_recursive() {
        let n = 10;
        let a = 55;

        assert_eq!(fibo_recursive(n), a);

        let mut memo = vec![None; n + 1];
        assert_eq!(fibo_dynamic(n, &mut memo), a);
    }

    /// ### 문제
    ///
    /// 2 * n 크기의 직사각형을 1 * 2, 2 * 1 타일로 채우는 방법의 수를 구하는 프로그램을 작성하시오.
    ///
    /// ### 입력
    ///
    /// 첫째 줄에 n이 주어진다 `1 <= n <= 1,000`
    #[test]
    fn test_n_times_2_tiling() {
        let n: usize = 9;
        let mut memo = vec![0; n + 1];
        memo[1] = 1;
        memo[2] = 2;

        for i in 3..=n {
            memo[i] = memo[i - 1] + memo[i - 2];
        }

        assert_eq!(memo[n], 55);
    }

    /// ### 문제
    ///
    /// 삼각형이 나선 모양으로 놓여져 있다. 첫 삼각형은 정삼각형으로 변의 길이는 1이다.
    /// 그 다음에는 다음과 같은 과정으로 정삼각형을 계속 추가한다.
    /// 나선에서 가장 긴 변의 길이를 k라 했을 때, 그 변에 길이가 k인 정삼각형을 추가한다.
    /// 파도반 수열 P(N)은 나선에 있는 정삼각형의 변의 길이이다.
    /// P(1)부터 P(10)까지 첫 10개 숫자는 1, 1, 1, 2, 2, 3, 4, 5, 7, 9이다.
    /// N이 주어졌을 때, P(N)을 구하는 프로그램을 작성하시오.
    ///
    /// ### 입력
    ///
    /// 첫째 줄에 테스트 케이스의 개수 T가 주어진다. 각 테스트 케이스는 한 줄로 이루어져 있고, N이 주어진다. `1 ≤ N ≤ 100`
    #[test]
    fn test_padovan_sequence() {
        let n = 12;
        let mut a = 1;

        if n > 3 {
            let mut p = vec![0; n + 1];
            p[1] = 1;
            p[2] = 1;
            p[3] = 1;

            for i in 3..=n {
                p[i] = p[i - 2] + p[i - 3];
            }

            a = p[n];
        }

        assert_eq!(a, 16);
    }
}
