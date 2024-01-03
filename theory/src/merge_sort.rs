#[cfg(test)]
mod tests {
    use log::debug;

    fn merge(left: Vec<u32>, right: Vec<u32>) -> Vec<u32> {
        let mut res = Vec::new();
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            let (l, r) = (left[i].clone(), right[j].clone());

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

    fn merge_sort(v: Vec<u32>) -> Vec<u32> {
        let len = v.len();
        if len <= 1 {
            return v;
        }

        let mid = len / 2;
        let left = merge_sort(v[..mid].to_vec());
        let right = merge_sort(v[mid..].to_vec());

        merge(left, right)
    }

    #[test]
    fn test_merge_sort() {
        let before = vec![38, 27, 43, 3, 9, 82, 10];
        let after = merge_sort(before.clone());

        debug!("before: {:?}", before);
        debug!("after: {:?}", after);

        assert_eq!(after, vec![3, 9, 10, 27, 38, 43, 82]);
    }
}
