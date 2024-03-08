#[cfg(test)]
mod test {
    // 백준 2751
    #[test]
    fn sort_number2() {
        let input = vec![7, 6, 5, 4, 8, 9, 10, 3, 2, 1];
        let mut answer = input.clone();
        answer.sort();

        assert_eq!(solution(input), answer);
    }

    fn merge(left: Vec<u32>, right: Vec<u32>) -> Vec<u32> {
        let (left_len, right_len) = (left.len(), right.len());
        let (mut i, mut j) = (0, 0);
        let mut res = Vec::with_capacity(left_len + right_len);

        while i < left_len && j < right_len {
            let (l, r) = (left[i], right[j]);

            if l < r {
                res.push(l);
                i += 1;
            } else {
                res.push(r);
                j += 1;
            }
        }

        log::debug!(
            "\n{:?}{:?}\n{:?}\n{:?}{:?}",
            &left,
            &right,
            res,
            &left[i..],
            &right[j..]
        );

        res.extend_from_slice(&left[i..]);
        res.extend_from_slice(&right[j..]);

        log::debug!("{res:?}");

        res
    }

    fn solution(vec: Vec<u32>) -> Vec<u32> {
        let len = vec.len();

        if len <= 1 {
            return vec;
        }

        let mid = len / 2;
        let left = solution(vec[0..mid].to_vec());
        let right = solution(vec[mid..len].to_vec());

        merge(left, right)
    }
}
