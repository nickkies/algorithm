#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    // 백준 1966번
    #[test]
    fn queue_printer() {
        let input1 = (1, 0, "4 2".to_string());
        let input2 = (4, 2, "1 2 3 4".to_string());
        let input3 = (6, 0, "1 1 9 1 1 1".to_string());

        assert_eq!(solution(input1), 1);
        assert_eq!(solution(input2), 2);
        assert_eq!(solution(input3), 5);
    }

    fn solution((_, target, input): (usize, usize, String)) -> usize {
        let mut queue = input
            .split_whitespace()
            .enumerate()
            .map(|(i, p)| (p.parse::<u8>().unwrap(), target == i))
            .collect::<VecDeque<(u8, bool)>>();

        let mut i = 1;
        while let Some((p, bool)) = queue.pop_front() {
            let mut ok = false;
            for j in 0..queue.len() {
                let (t, _) = queue.get(j).unwrap();
                if p < *t {
                    queue.push_back((p, bool));
                    ok = false;
                    break;
                }
                ok = true;
            }

            if ok {
                if bool {
                    return i;
                }
                i += 1;
            }
        }

        0
    }
}
