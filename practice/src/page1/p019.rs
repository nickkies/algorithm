#[cfg(test)]
mod test {
    // 백준 1543
    #[test]
    fn search_document() {
        let input1 = ("ababababa".to_string(), "aba".to_string());
        let input2 = ("a a a a a".to_string(), "a a".to_string());
        let input3 = ("ababababa".to_string(), "ababa".to_string());
        let input4 = ("aaaaaaa".to_string(), "aa".to_string());

        assert_eq!(solution(input1), 2);
        assert_eq!(solution(input2), 2);
        assert_eq!(solution(input3), 1);
        assert_eq!(solution(input4), 3);
    }

    fn solution((target, str): (String, String)) -> usize {
        let vec = target.split(&str).collect::<Vec<&str>>();

        vec.len() - 1
    }
}
