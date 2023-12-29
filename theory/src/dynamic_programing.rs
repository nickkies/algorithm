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
}
