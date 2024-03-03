#[cfg(test)]
mod test {
    // 백준 10989
    #[test]
    fn sort_number3() {
        let input = "5 2 3 1 4 2 3 5 1 7";
        assert_eq!(solution(input), vec![1, 1, 2, 2, 3, 3, 4, 5, 5, 7]);
    }

    fn solution(str: &str) -> Vec<u32> {
        let mut arr = [0; 10001];
        str.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .for_each(|x| {
                arr[*x as usize] += 1;
            });

        let mut res = vec![];

        arr.iter().enumerate().for_each(|(i, &x)| {
            if x != 0 {
                for _ in 0..x {
                    res.push(i as u32);
                }
            }
        });

        res
    }
}
