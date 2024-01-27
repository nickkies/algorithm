#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap, HashSet};

    use log::debug;

    fn prim(graph: HashMap<char, Vec<(char, usize)>>, start_node: char) -> Option<usize> {
        let mut visited = HashSet::from([start_node]);
        let mut min_heap = BTreeMap::new();
        let mut sum = 0;
        let mut remain = graph.len();

        let start_edges = graph.get(&start_node);

        assert!(start_edges.is_some());

        start_edges.unwrap().iter().for_each(|(next_node, weight)| {
            min_heap.insert(weight, (start_node, *next_node));
        });

        while let Some((weight, (current_node, next_node))) = min_heap.pop_first() {
            if visited.contains(&next_node) {
                continue;
            }

            debug!("({current_node}, {next_node}): {weight}");

            visited.insert(next_node.clone());
            sum += weight;
            remain -= 1;

            let next_edges = graph.get(&next_node);

            assert!(next_edges.is_some());

            next_edges.unwrap().iter().for_each(|(node, weight)| {
                if !visited.contains(&node) {
                    min_heap.insert(weight, (next_node, *node));
                }
            })
        }

        if remain == 1 {
            Some(sum)
        } else {
            None
        }
    }

    #[test]
    fn test_prim() {
        let graph = HashMap::from([
            ('A', vec![('B', 7), ('D', 5)]),
            ('B', vec![('A', 7), ('C', 8), ('D', 9), ('E', 7)]),
            ('C', vec![('B', 8), ('E', 5)]),
            ('D', vec![('A', 5), ('B', 9), ('E', 7), ('F', 6)]),
            ('E', vec![('B', 7), ('C', 5), ('D', 7), ('F', 8), ('G', 9)]),
            ('F', vec![('D', 6), ('E', 8), ('G', 11)]),
            ('G', vec![('E', 9), ('F', 11)]),
        ]);

        let result = prim(graph, 'A');

        assert!(result.is_some());
        assert_eq!(result.unwrap(), 39);
    }
}
