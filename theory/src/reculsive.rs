#[cfg(test)]
mod tests {
    fn factorial(num: usize) -> usize {
        if num <= 1 {
            num
        } else {
            num * factorial(num - 1)
        }
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), 3_628_800);
    }
}
