#[cfg(test)]
mod test {
    use std::collections::HashMap;

    // 백준 4195
    #[test]
    fn friend_network() {
        let input1 = ["Fred Barney", "Barney Betty", "Betty Wilma"];
        let input2 = ["Fred Barney", "Betty Wilma", "Barney Betty"];

        assert_eq!(solution(&input1), vec![2, 3, 4]);
        assert_eq!(solution(&input2), vec![2, 2, 4]);
    }

    fn solution(input: &[&str]) -> Vec<usize> {
        let mut res = vec![];
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut number: HashMap<String, usize> = HashMap::new();

        for &items in input {
            let (x, y) = {
                let vec = items.split_whitespace().collect::<Vec<&str>>();
                (vec[0].to_string(), vec[1].to_string())
            };

            parent.entry(x.clone()).or_insert_with(|| x.clone());
            parent.entry(y.clone()).or_insert_with(|| y.clone());

            number.entry(x.clone()).or_insert(1);
            number.entry(y.clone()).or_insert(1);

            let x_root = find(&mut parent, &x);
            let y_root = find(&mut parent, &y);

            if x_root != y_root {
                parent.insert(y_root.clone(), x_root.clone());
                *number.get_mut(&x_root).unwrap() += number[&y_root];
            }

            res.push(number[&find(&mut parent, &x)]);

            log::debug!("parent: {parent:?}");
            log::debug!("number: {number:?}");
        }

        res
    }

    fn find(parent: &mut HashMap<String, String>, x: &String) -> String {
        if x == &parent[x] {
            x.clone()
        } else {
            let p = find(parent, &parent[x].clone());
            parent.insert(x.clone(), p.clone());
            p
        }
    }
}
