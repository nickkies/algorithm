#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use log::debug;
    use rand::seq::SliceRandom;

    fn insertion_sort<T>(v: &mut [T])
    where
        T: Debug + PartialOrd,
    {
        debug!("before: {:?}", &v[0..4]);

        for i in 1..v.len() {
            let mut j = i;
            while j > 0 && v[j - 1] > v[j] {
                v.swap(j - 1, j);
                j -= 1;
            }
        }

        debug!("after: {:?}", &v[0..4]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut v: [usize; 4] = [1, 9, 3, 2];

        insertion_sort(&mut v);

        assert_eq!(v, [1, 2, 3, 9]);
    }

    #[test]
    fn test_insertion_sort2() {
        let mut v = (1..=10_000).collect::<Vec<usize>>();
        let answer = v.clone();

        v.shuffle(&mut rand::thread_rng());

        insertion_sort(&mut v);

        assert_eq!(v, answer);
    }
}
