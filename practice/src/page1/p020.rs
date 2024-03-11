#[cfg(test)]
mod test {
    // 백준 1568
    #[test]
    fn bird() {
        assert_eq!(solution(14), 7);
        assert_eq!(solution(1), 1);
        assert_eq!(solution(3), 2);
        assert_eq!(solution(4), 3);
        assert_eq!(solution(100), 18);
    }

    fn solution(mut n: usize) -> usize {
        let mut res = 0;
        let mut k = 1;

        while n > 0 {
            if n < k {
                k = 1;
            }

            n -= k;
            k += 1;
            res += 1;
        }

        res
    }
}
