#[cfg(test)]
mod test {
    // 백준 10814
    #[test]
    fn sort_by_age() {
        let mut input = vec!["21 Junkyu", "21 Dohyun", "20 Sunyoung"];

        solution(&mut input);

        assert_eq!(input, vec!["20 Sunyoung", "21 Junkyu", "21 Dohyun"])
    }

    fn solution(input: &mut Vec<&str>) {
        input.sort_by(|&a, &b| {
            let num_a = a
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let num_b = b
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            num_a.cmp(&num_b)
        });
    }
}
