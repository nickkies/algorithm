#[cfg(test)]
mod test {
    // 백준 2747
    #[test]
    fn fibonacci_sequence() {
        assert_eq!(solution(10), 55);
        assert_eq!(solution(17), 1597);
    }

    fn solution(num: u32) -> u32 {
        if num < 2 {
            return num;
        }

        solution(num - 1) + solution(num - 2)
    }
}
