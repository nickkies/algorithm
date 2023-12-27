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
}
