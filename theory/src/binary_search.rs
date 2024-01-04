#[cfg(test)]
mod tests {
    fn binary_search(arr: &[u32], target: u32) -> Option<usize> {
        let mut low = 0;
        let mut high = arr.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            let mid_value = arr[mid];

            if mid_value == target {
                return Some(mid);
            } else if mid_value < target {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        None
    }

    #[test]
    fn test_binary_search() {
        let sorted_array = vec![1, 2, 3, 4, 5, 7, 8, 9, 10];
        let target = 7;

        assert_eq!(binary_search(&sorted_array, target), Some(5));

        let target = 6;
        assert_eq!(binary_search(&sorted_array, target), None);
    }
}
