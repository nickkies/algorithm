#[cfg(test)]
mod test {
    // 백준 1668
    #[test]
    fn trophy_display() {
        assert_eq!(solution(vec![1, 2, 3, 4, 5]), (5, 1));
        assert_eq!(solution(vec![5, 2, 3, 4, 5]), (1, 1));
        assert_eq!(solution(vec![1, 2, 5, 2, 1]), (3, 3));
        assert_eq!(solution(vec![1, 4, 2, 5, 3, 7, 1]), (4, 2));
    }

    fn solution(mut vec: Vec<u8>) -> (u8, u8) {
        let left = count(&vec);

        vec.reverse();

        let right = count(&vec);

        (left, right)
    }

    fn count(vec: &Vec<u8>) -> u8 {
        let (cnt, _) = vec.iter().fold(
            (0, 0),
            |(cnt, max), &x| {
                if x > max {
                    (cnt + 1, x)
                } else {
                    (cnt, max)
                }
            },
        );

        cnt
    }
}
