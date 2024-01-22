#[cfg(test)]
mod tests {
    use log::debug;
    use std::collections::{BTreeMap, HashMap, HashSet};

    type Graph = HashMap<char, HashMap<char, usize>>;

    fn dijkstra(graph: Graph, (start, end): (char, char)) -> Option<usize> {
        let mut distances = HashMap::from([(start, 0)]);
        let mut visited: HashSet<char> = HashSet::new();
        let mut priority_queue = BTreeMap::from([(0, start)]);

        while let Some((distance, node)) = priority_queue.pop_first() {
            if node == end {
                debug!("graph: {graph:?}");
                debug!("distances: {distances:?}");
                debug!("visited: {visited:?}");
                debug!("priority_queue: {priority_queue:?}");

                return Some(distance);
            }

            if visited.contains(&node) {
                continue;
            }

            visited.insert(node);

            graph.get(&node)?.iter().for_each(|(neighber, weight)| {
                let new_distance = distance + weight;
                if new_distance < *distances.get(neighber).unwrap_or(&usize::MAX) {
                    distances.insert(*neighber, new_distance);
                    priority_queue.insert(new_distance, *neighber);
                }
            });
        }

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
