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

    /// ### 부분 배낭 문제
    ///
    /// - 무게 제한이 30인 배낭에 최대 가치를 가지도록 물건을 넣는 문제
    /// - 각 물건은 무게(w)와 가치(v)로 표현될 수 있음
    /// - 물건은 쪼갤 수 있으므로 물건의 일부분이 배낭에 넣어질 수 있음, 그래서 Fractional Knapsack Problem 으로 부름
    /// - Fractional Knapsack Problem 의 반대로 물건을 쪼개서 넣을 수 없는 배낭 문제도 존재함 (0/1 Knapsack Problem 으로 부름)
    #[test]
    fn fractional_knapsack() {
        let mut items = vec![(10, 10), (15, 12), (20, 10), (25, 8), (30, 5)];
        let mut capacity = 30;
        let mut total_value = 0.0;
        let mut details = vec![];

        items.sort_by(|(a_w, a_v), (b_w, b_v)| {
            let a = *a_v as f32 / *a_w as f32;
            let b = *b_v as f32 / *b_w as f32;
            b.partial_cmp(&a).unwrap()
        });

        debug!("sorted items: {items:?}");

        for (weight, value) in items {
            if capacity - weight >= 0 {
                capacity -= weight;
                total_value += value as f32;
                details.push((weight, value, 1.0));
            } else {
                let fraction = capacity as f32 / weight as f32;
                total_value += value as f32 * fraction;
                details.push((weight, value, fraction));
                break;
            }
        }

        debug!("details: {details:?}");

        assert_eq!(total_value, 24.5);
    }
}
