#[cfg(test)]
mod tests {
    use log::debug;

    fn quick_sort(v: &mut [u32]) {
        let len = v.len();

        if len <= 1 {
            return;
        }

        let pivot_idx = len / 2;
        v.swap(pivot_idx, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if v[j] <= v[len - 1] {
                v.swap(i, j);
                i += 1;
            }
        }

        v.swap(i, len - 1);

        quick_sort(&mut v[0..i]);
        quick_sort(&mut v[i + 1..]);
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 2, 9, 7, 5, 1, 8, 3, 6];
        debug!("before: {:?}", &v);

        quick_sort(&mut v);

        debug!("after: {:?}", &v);

        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
