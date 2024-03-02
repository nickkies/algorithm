#[cfg(test)]
mod test {
    // 백준 11650
    #[test]
    fn arrange_coordinates() {
        let mut input = vec![(3, 4), (1, 1), (1, -1), (2, 2), (3, 3)];

        solution(&mut input);
        assert_eq!(input, vec![(1, -1), (1, 1), (2, 2), (3, 3), (3, 4)]);
    }

    fn solution(points: &mut Vec<(i32, i32)>) {
        points.sort_by(|a, b| {
            let (a_x, a_y) = a;
            let (b_x, b_y) = b;

            if a_x != b_x {
                a_x.cmp(&b_x)
            } else {
                a_y.cmp(&b_y)
            }
        });
    }
}
