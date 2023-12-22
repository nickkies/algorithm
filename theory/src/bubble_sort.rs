#[cfg(test)]
mod tests {
    use log::debug;
    use rand::seq::SliceRandom;

    fn bubble_sort<T>(v: &mut [T])
    where
        T: std::fmt::Debug + std::cmp::PartialOrd + Copy,
    {
        let len = v.len();

        debug!("before: {v:?}");

        for i in 0..len {
            for j in 0..len - i - 1 {
                let (prev, next) = (v[j], v[j + 1]);
                if prev > next {
                    v[j] = next;
                    v[j + 1] = prev;
                }
            }
        }

        debug!("after: {v:?}");
    }

    fn bubble_sort2(v: &mut [usize]) {
        let len = v.len();

        debug!("before: {:?}", &v[0..10]);

        for i in 0..len {
            for j in 0..len - i - 1 {
                if v[j] > v[j + 1] {
                    v.swap(j, j + 1);
                }
            }
        }

        debug!("after: {:?}", &v[0..10]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = [1, 9, 3, 2];

        bubble_sort(&mut v);

        assert_eq!(v, [1, 2, 3, 9]);
    }

    #[test]
    fn test_bubble_sort2() {
        let mut v = (1..=10_000).collect::<Vec<usize>>();
        let answer = v.clone();

        v.shuffle(&mut rand::thread_rng());

        bubble_sort2(&mut v);

        assert_eq!(v, answer);
    }
}
