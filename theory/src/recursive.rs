#[cfg(test)]
mod tests {

    fn factorial(num: usize) -> usize {
        if num <= 1 {
            num
        } else {
            num * factorial(num - 1)
        }
    }

    fn palindrome(s: &str) -> bool {
        let mut str = s.chars();
        let cnt = str.clone().count();

        if cnt <= 1 {
            return true;
        }

        if str.next().unwrap() == str.last().unwrap() {
            let s = s.chars().skip(1).take(cnt - 2).collect::<String>();
            palindrome(&s)
        } else {
            false
        }
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), 3_628_800);
    }

    #[test]
    fn test_palindrome() {
        let s = "hello";
        assert!(!palindrome(s));

        let s = "level";
        assert!(palindrome(s));
    }
}
