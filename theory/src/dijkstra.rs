#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    type Graph = HashMap<char, HashMap<char, usize>>;

    fn dijkstra(graph: Graph, (start, end): (char, char)) -> Option<usize> {
        None
    }

    #[test]
    fn test_dijkstra() {
        let graph = HashMap::from([
            ('A', HashMap::from([('B', 8), ('C', 1), ('D', 2)])),
            ('B', HashMap::new()),
            ('C', HashMap::from([('B', 5), ('D', 2)])),
            ('D', HashMap::from([('E', 3), ('F', 5)])),
            ('E', HashMap::from([('F', 1)])),
            ('F', HashMap::from([('A', 5)])),
        ]);
        let node_info = ('A', 'F');

        let answer = dijkstra(graph, node_info);

        assert!(answer.is_some());
        assert_eq!(answer.unwrap(), 6);
    }
}
