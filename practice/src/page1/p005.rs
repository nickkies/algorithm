#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    // 백준 5397번
    #[test]
    fn key_logger() {
        let input1 = "<<BP<A>>Cd-";
        let input2 = "ThIsIsS3Cr3t";

        assert_eq!(solution(input1), "BAPC".to_string());
        assert_eq!(solution(input2), "ThIsIsS3Cr3t".to_string());
    }

    fn solution(str: &str) -> String {
        let mut left_queue: VecDeque<char> = VecDeque::new();
        let mut right_queue: VecDeque<char> = VecDeque::new();

        let chars = str.chars().collect::<Vec<char>>();

        chars.into_iter().for_each(|c| match c {
            '<' => {
                if let Some(char) = left_queue.pop_back() {
                    right_queue.push_front(char);
                }
            }
            '>' => {
                if let Some(char) = right_queue.pop_front() {
                    left_queue.push_back(char);
                }
            }
            '-' => {
                left_queue.pop_back();
            }
            _ => {
                left_queue.push_back(c);
            }
        });

        left_queue
            .into_iter()
            .chain(right_queue.into_iter())
            .collect::<String>()
    }
}
