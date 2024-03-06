#[cfg(test)]
mod test {
    // 백준 1074
    #[test]
    fn z() {
        let input1 = "2 3 1";
        let input2 = "3 7 7";
        let input3 = "1 0 0";
        let input4 = "4 7 7";
        let input5 = "10 511 511";
        let input6 = "10 512 512";

        assert_eq!(solution(input1), 11);
        assert_eq!(solution(input2), 63);
        assert_eq!(solution(input3), 0);
        assert_eq!(solution(input4), 63);
        assert_eq!(solution(input5), 262143);
        assert_eq!(solution(input6), 786432);
    }

    fn solution(input: &str) -> usize {
        let arr = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut res = 0;
        let (n, r, c) = (arr[0], arr[1], arr[2]);

        recursive(1 << n, 0, 0, &mut res, r, c);

        res
    }

    fn recursive(n: usize, x: usize, y: usize, res: &mut usize, r: usize, c: usize) {
        if n == 2 {
            if x == r && y == c {
                return;
            }

            *res += 1;

            if x == r && y + 1 == c {
                return;
            }

            *res += 1;

            if x + 1 == r && y == c {
                return;
            }

            *res += 1;

            if x + 1 == r && y + 1 == c {
                return;
            }

            *res += 1;

            return;
        }

        let m = n / 2;

        if r < x + m && c < y + m {
            recursive(m, x, y, res, r, c);
        } else if r < x + m && c >= y + m {
            *res += m * m;
            recursive(m, x, y + m, res, r, c);
        } else if r >= x + m && c < y + m {
            *res += 2 * m * m;
            recursive(m, x + m, y, res, r, c);
        } else {
            *res += 3 * m * m;
            recursive(m, x + m, y + m, res, r, c);
        }
    }
}
