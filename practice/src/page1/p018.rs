#[cfg(test)]
mod test {
    // 백준 11004
    #[test]
    fn kth_number() {
        let (vec, k) = (vec![4, 1, 2, 3, 5], 2);
        assert_eq!(solution(vec, k), 2);
    }

    fn recursive(vec: Vec<u32>) -> Vec<u32> {
        let len = vec.len();

        if len <= 1 {
            return vec;
        }

        let mid = len / 2;
        let left = recursive(vec[0..mid].to_vec());
        let right = recursive(vec[mid..].to_vec());

        let (left_len, right_len) = (left.len(), right.len());
        let (mut i, mut j) = (0, 0);
        let mut res = vec![];

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

        res.extend_from_slice(&left[i..]);
        res.extend_from_slice(&right[j..]);

        res
    }

    fn solution(vec: Vec<u32>, k: usize) -> u32 {
        let sorted = recursive(vec);

        log::debug!("{sorted:?}");

        sorted[k - 1]
    }
}
