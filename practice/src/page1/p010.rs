#[cfg(test)]
mod test {

    trait SortReverse {
        fn sort_reverse(&mut self);
    }

    impl SortReverse for Vec<usize> {
        fn sort_reverse(&mut self) {
            self.sort_by(|a, b| b.cmp(a));
        }
    }

    // 백준 1427
    #[test]
    fn sort_inside() {
        let input1 = vec![2, 1, 4, 3];
        let input2 = vec![9, 9, 9, 9, 9, 8, 9, 9, 9];
        let input3 = vec![6, 1, 4, 2, 3];
        let input4 = vec![5, 0, 0, 6, 1, 3, 0, 0, 9];
        let mut answer1 = input1.clone();
        let mut answer2 = input2.clone();
        let mut answer3 = input3.clone();
        let mut answer4 = input4.clone();
        answer1.sort_reverse();
        answer2.sort_reverse();
        answer3.sort_reverse();
        answer4.sort_reverse();

        assert_eq!(merge_sort(input1), answer1);
        assert_eq!(merge_sort(input2), answer2);
        assert_eq!(merge_sort(input3), answer3);
        assert_eq!(merge_sort(input4), answer4);
        assert!(true)
    }

    fn merge(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
        let (left_len, right_len) = (left.len(), right.len());
        let mut res = Vec::with_capacity(left_len + right_len);
        let (mut i, mut j) = (0, 0);

        while i < left_len && j < right_len {
            let (l, r) = (left[i], right[j]);

            if l > r {
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

    fn merge_sort(arr: Vec<usize>) -> Vec<usize> {
        let len = arr.len();

        if len <= 1 {
            return arr;
        }

        let mid = len / 2;
        let left = merge_sort(arr[..mid].to_vec());
        let right = merge_sort(arr[mid..].to_vec());

        merge(left, right)
    }
}
