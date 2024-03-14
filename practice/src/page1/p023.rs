#[cfg(test)]
mod test {
    // 백준 1236
    #[test]
    fn protecting_the_castle() {
        let input1 = vec![
            vec![".", ".", ".", "."],
            vec![".", ".", ".", "."],
            vec![".", ".", ".", "."],
            vec![".", ".", ".", "."],
        ];
        let input2 = vec![
            vec!["X", "X", ".", ".", "."],
            vec![".", "X", "X", ".", "."],
            vec![".", ".", ".", "X", "X"],
        ];
        let input3 = vec![
            vec![".", ".", ".", ".", "X", "X", "X", "X"],
            vec![".", ".", ".", ".", ".", ".", ".", "."],
            vec!["X", "X", ".", "X", ".", "X", "X", "."],
            vec![".", ".", ".", ".", ".", ".", ".", "."],
            vec![".", ".", ".", ".", ".", ".", ".", "."],
        ];

        assert_eq!(solution(input1), 4);
        assert_eq!(solution(input2), 0);
        assert_eq!(solution(input3), 3);
    }

    fn solution(vv: Vec<Vec<&str>>) -> u8 {
        let (n, m) = (vv.len(), vv[0].len());

        let mut row = vec![true; n];
        let mut col = vec![true; m];

        vv.iter().enumerate().for_each(|(i, v)| {
            v.iter().enumerate().for_each(|(j, &x)| {
                if x == "X" {
                    row[i] = false;
                    col[j] = false;
                }
            })
        });

        log::debug!("{row:?}");
        log::debug!("{col:?}");

        let row_cnt = row.iter().fold(0, |acc, &x| if x { acc + 1 } else { acc });
        let col_cnt = col.iter().fold(0, |acc, &x| if x { acc + 1 } else { acc });

        std::cmp::max(row_cnt, col_cnt)
    }
}
