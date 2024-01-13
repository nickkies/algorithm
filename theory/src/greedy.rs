#[cfg(test)]
mod tests {
    use log::debug;

    /// ### 동전 문제
    ///
    /// ##### 지불해야 하는 값이 `4720`원 일 때 `10원 50원 100원, 500원 동전`으로 동전의 수가 가장 적게 지불하시오.
    #[test]
    fn min_coin_count() {
        let (mut total, mut coins) = (4720, vec![10, 100, 50, 500]);
        let mut sum = 0;

        coins.sort();
        coins.reverse();

        let detail = coins
            .iter()
            .map(|&coin| {
                let cnt = total / coin;
                sum += cnt;
                total -= cnt * coin;

                (coin, cnt)
            })
            .collect::<Vec<(usize, usize)>>();

        debug!("{detail:?}");

        assert_eq!(sum, 13);
    }
}
