#[cfg(test)]
mod tests {
    use log::debug;
    use rand::Rng;

    /// ### 문제
    ///
    /// 체스판의 크기가 자연수 N×N이고 퀸이 서로 공격할 수 없는 배치를 찾아라.
    /// 각 퀸은 같은 행, 열, 또는 대각선에 있으면 서로 공격할 수 있다.
    #[test]
    fn test_n_queens() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(4..=10);

        let mut result: Vec<Vec<usize>> = vec![];
        dfs(n, 0, &mut Vec::new(), &mut result);

        assert!(!result.is_empty());
    }

    fn is_available(candidate: &Vec<usize>, current_col: usize) -> bool {
        let current_row = candidate.len();

        for (queen_row, &queen_col) in candidate.iter().enumerate() {
            if queen_col == current_col
                || (queen_col as isize - current_col as isize).abs()
                    == (current_row as isize - queen_row as isize)
            {
                return false;
            }
        }

        true
    }

    fn dfs(
        n: usize,
        current_row: usize,
        current_candidate: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if current_row == n {
            debug!("{current_candidate:?}");
            result.push(current_candidate.clone());
            return;
        }

        (0..n).for_each(|candidate_col| {
            if is_available(current_candidate, candidate_col) {
                current_candidate.push(candidate_col);

                dfs(n, current_row + 1, current_candidate, result);
                current_candidate.pop();
            }
        });
    }
}
