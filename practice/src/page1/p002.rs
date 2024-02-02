#[cfg(test)]
mod tests {
    /// N(3 ≤ N ≤ 100)개의 숫자 중에서
    /// 특정 N개의 숫자가 주어지며,
    /// M(10 ≤ M ≤ 300,000)을 넘지 않으면서 M에 최대한 가까운
    /// 3개의 숫자의 합
    #[test]
    fn blackjack() {
        let input1 = (5, 21, "5 6 7 8 9".to_string());
        let input2 = (10, 500, "93 181 245 214 315 36 185 138 216 295".to_string());

        assert_eq!(solution(input1), 21);
        assert_eq!(solution(input2), 497);
    }

    fn solution((_, m, str): (usize, u32, String)) -> u32 {
        let nums = str
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let mut sum = 0;

        for (i, &num_i) in nums.iter().enumerate() {
            if num_i > m {
                continue;
            }

            for (j, &num_j) in nums.iter().skip(i + 1).enumerate() {
                let sum2 = num_i + num_j;

                if sum2 > m {
                    continue;
                }

                for &num_k in nums.iter().skip(j + 1) {
                    let sum3 = sum2 + num_k;

                    if sum3 > m {
                        continue;
                    } else if sum3 == m {
                        return sum3;
                    } else if sum3 > sum {
                        sum = sum3;
                    }
                }
            }
        }

        sum
    }
}
