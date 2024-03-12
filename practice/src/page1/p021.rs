#[cfg(test)]
mod test {
    use std::collections::HashMap;

    // 백준 1302
    #[test]
    fn best_seller() {
        let input1 = vec!["top", "top", "top", "top", "kimtop"];
        let input2 = vec![
            "table", "chair", "table", "table", "lamp", "door", "lamp", "table", "chair",
        ];
        let input3 = vec!["a", "a", "a", "b", "b", "b"];
        let input4 = vec![
            "icecream",
            "peanuts",
            "peanuts",
            "chocolate",
            "candy",
            "chocolate",
            "icecream",
            "apple",
        ];
        let input5 = vec!["soul"];

        assert_eq!(solution(input1), "top");
        assert_eq!(solution(input2), "table");
        assert_eq!(solution(input3), "a");
        assert_eq!(solution(input4), "chocolate");
        assert_eq!(solution(input5), "soul");
    }

    fn solution(books: Vec<&str>) -> &str {
        let mut map: HashMap<&str, u8> = HashMap::new();
        let mut vec = vec![];

        books.iter().for_each(|&book| {
            map.entry(book).and_modify(|cnt| *cnt += 1).or_insert(0);
        });

        let max = map.values().max().expect("No books");

        map.iter().for_each(|(book, cnt)| {
            if cnt == max {
                vec.push(*book);
            }
        });

        vec.sort();

        vec[0]
    }
}
