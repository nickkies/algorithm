#[cfg(test)]
mod test {
    use std::collections::HashSet;

    // 백준 1920
    #[test]
    fn find_number() {
        let (target, input) = ("4 1 5 2 3", "1 3 7 9 5");

        assert_eq!(solution(target, input), vec![1, 1, 0, 0, 1]);
    }

    fn solution(target: &str, input: &str) -> Vec<u8> {
        let mut res = vec![];
        let targets = target
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<HashSet<u8>>();
        let inputs = input
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        inputs.iter().for_each(|x| {
            let mut num = 0;
            if let Some(_) = targets.get(x) {
                num = 1;
            }
            res.push(num);
        });

        res
    }
}
