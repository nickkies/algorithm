#[cfg(test)]
mod tests {
    fn recursive(n: usize) -> usize {
        if n <= 1 {
            n
        } else {
            recursive(n - 1) + recursive(n - 2)
        }
    }

    fn dynamic(n: usize) -> usize {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return dynamic(n - 1) + dynamic(n - 2);
        }
    }

    #[test]
    fn test_recursive() {
        assert_eq!(recursive(10), 55);
        assert_eq!(dynamic(10), 55);
    }
}
