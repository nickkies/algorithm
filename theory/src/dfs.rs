#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, VecDeque};

    fn dfs(graph: &HashMap<char, Vec<char>>, start_node: char) -> Vec<char> {
        let mut visited = Vec::new();
        let mut need_visit = VecDeque::from([start_node]);

        while let Some(node) = need_visit.pop_back() {
            if !visited.contains(&node) {
                visited.push(node);
                need_visit.extend(graph.get(&node).unwrap());
            }
        }

        visited
    }

    fn dfs_count(graph: &HashMap<char, Vec<char>>, start_node: char) -> usize {
        let mut visited = HashSet::new();
        let mut need_visit = VecDeque::from([start_node]);
        let mut count = 0;

        while let Some(node) = need_visit.pop_back() {
            if !visited.contains(&node) {
                visited.insert(node);
                need_visit.extend(graph.get(&node).unwrap());
            }

            count += 1;
        }

        count
    }

    #[test]
    fn test_dfs() {
        let graph = HashMap::from([
            ('A', vec!['B', 'C']),
            ('B', vec!['A', 'D']),
            ('C', vec!['A', 'G', 'H', 'I']),
            ('D', vec!['B', 'E', 'F']),
            ('E', vec!['D']),
            ('F', vec!['D']),
            ('G', vec!['C']),
            ('H', vec!['C']),
            ('I', vec!['C', 'J']),
            ('J', vec!['I']),
        ]);

        assert_eq!(
            dfs(&graph, 'A'),
            vec!['A', 'C', 'I', 'J', 'H', 'G', 'B', 'D', 'F', 'E']
        );

        assert_eq!(dfs_count(&graph, 'A'), 19);
    }
}
