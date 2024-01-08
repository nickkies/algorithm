#[cfg(test)]
mod tests {
    fn sequential_search(items: &[u32], target: u32) -> Option<usize> {
        for (idx, item) in items.iter().enumerate() {
            if *item == target {
                return Some(idx);
            }
        }
        None
    }

    #[test]
    fn test_sequential_search() {
        let items = vec![1, 2, 3, 4, 5, 7, 8, 9, 10];
        let target = 7;

        assert_eq!(sequential_search(&items, target), Some(5));

        let target = 6;
        assert_eq!(sequential_search(&items, target), None);
    }
}
