#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use log::debug;
    use rand::seq::SliceRandom;

    fn selection_sort<T>(v: &mut [T])
    where
        T: Debug + PartialOrd,
    {
        debug!("before: {:?}", &v[0..4]);

        let n = v.len();

        for i in 0..n {
            let mut min_idx = i;

            for j in (i + 1)..n {
                if v[j] < v[min_idx] {
                    min_idx = j;
                }
            }

            v.swap(i, min_idx);
        }

        debug!("after: {:?}", &v[0..4]);
    }

    #[test]
    fn test_selection_sort() {
        let mut v: [usize; 4] = [1, 9, 3, 2];

        selection_sort(&mut v);

        assert_eq!(v, [1, 2, 3, 9]);
    }

    #[test]
    fn test_selection_sort2() {
        let mut v = (1..=10_000).collect::<Vec<usize>>();
        let answer = v.clone();

        v.shuffle(&mut rand::thread_rng());

        selection_sort(&mut v);

        assert_eq!(v, answer);
    }
}
