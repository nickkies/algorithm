#[cfg(test)]
mod test {
    // 백준 7490
    #[test]
    fn make_0() {
        assert_eq!(solution(3), ["1+2-3".to_string()]);

        let mut output = solution(7);
        let mut answer = [
            "1+2-3+4-5-6+7".to_string(),
            "1+2-3-4+5+6-7".to_string(),
            "1-2 3+4+5+6+7".to_string(),
            "1-2 3-4 5+6 7".to_string(),
            "1-2+3+4-5+6-7".to_string(),
            "1-2-3-4-5+6+7".to_string(),
        ];
        output.sort();
        answer.sort();

        log::debug!("{output:?}");
        log::debug!("{answer:?}");

        assert_eq!(output, answer);
    }

    fn solution(num: usize) -> Vec<String> {
        let ops = ['+', '-', ' '];
        let arr = (1..=num).collect::<Vec<usize>>();
        let mut res = vec![];

        // 모든 경우의 수
        let combinations = recursive(&ops, arr.len() - 1);

        log::debug!("{combinations:?}");

        combinations.iter().for_each(|combination| {
            let mut str = "".to_string(); // 최종 문자열
            let mut nums = vec![]; // 공백 처리한 숫자 배열
            let mut cur = arr[0]; // 공백 처리를 위한 변수

            // 공백처리
            // ex)
            // [1, 2, 3, 4], [" ", " ", " ", "+"]
            // [123, 4], ["+"]
            combination.iter().enumerate().for_each(|(i, op)| {
                if op.as_str() == " " {
                    cur = cur * 10 + arr[i + 1];
                } else {
                    nums.push(cur);
                    cur = arr[i + 1];
                }
            });

            nums.push(cur);

            // 공백을 뺀 배열 생성
            let ops = combination
                .iter()
                .filter(|&x| " " != x.as_str())
                .collect::<Vec<&String>>();

            // 연산
            let mut num = nums[0] as i32;
            ops.iter().enumerate().for_each(|(i, &op)| {
                if op.as_str() == "+" {
                    num = num + nums[i + 1] as i32;
                } else {
                    num = num - nums[i + 1] as i32;
                }
            });

            // 0 인지 검사
            if num == 0 {
                arr.iter().zip(combination).for_each(|(&num, op)| {
                    str = format!("{str}{num}{op}");
                });

                str = format!("{}{}", str, arr[arr.len() - 1]);

                res.push(str);
            }
        });

        res
    }

    fn recursive(ops: &[char], len: usize) -> Vec<Vec<String>> {
        if len == 0 {
            return vec![vec![]];
        }

        let mut combinations = Vec::new();

        for &op in ops {
            let smaller_combinations = recursive(ops, len - 1);
            for mut smaller_combination in smaller_combinations {
                smaller_combination.push(op.to_string());
                combinations.push(smaller_combination);
            }
        }

        combinations
    }
}
