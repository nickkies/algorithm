#[cfg(test)]
mod test {
    // 백준 2750
    #[test]
    fn sort_number() {
        let input = vec![5, 2, 3, 4, 1];
        let mut sorted = input.clone();
        sorted.sort();

        assert_eq!(quick_sort(input), sorted);
    }

    fn quick_sort(arr: Vec<usize>) -> Vec<usize> {
        let len: usize = arr.len();

        if len <= 1 {
            return arr;
        }

        let pivot = len / 2;
        let pivot_value = arr[pivot];

        let left = arr
            .iter()
            .filter(|&x| *x < pivot_value)
            .cloned()
            .collect::<Vec<usize>>();
        let middle = arr[pivot];
        let right = arr
            .iter()
            .filter(|&x| *x > pivot_value)
            .cloned()
            .collect::<Vec<usize>>();

        let mut res = vec![];

        res.extend(quick_sort(left));
        res.push(middle);
        res.extend(quick_sort(right));

        res
    }
}
