#[cfg(test)]
mod test {
    // 백준 2110
    #[test]
    fn 공유기_설치() {
        assert_eq!(solution(3, vec![1, 2, 8, 4, 9]), 3);
    }

    fn solution(c: usize, mut v: Vec<usize>) -> usize {
        v.sort();

        let mut start = v[1] - v[0];
        let mut end = v[v.len() - 1] - v[0];
        let mut res = 0;

        while start <= end {
            log::debug!("{start}, {end}");

            let mid = (start + end) / 2;
            let mut val = v[0];
            let mut cnt = 1;

            v.iter().skip(1).for_each(|&x| {
                if x >= val + mid {
                    val = x;
                    cnt += 1;
                }
            });

            if cnt >= c {
                start = mid + 1;
                res = mid;
            } else {
                end = mid - 1;
            }
        }

        res
    }
}
