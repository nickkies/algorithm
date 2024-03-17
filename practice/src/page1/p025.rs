#[cfg(test)]
mod test {
    // 백준 1939
    #[test]
    fn 중량제한() {
        let (start, end, v) = (1, 3, vec![vec![1, 2, 2], vec![3, 1, 3], vec![2, 3, 2]]);
        assert_eq!(solution(start, end, v), 3);
    }

    fn bfs(
        adj: &[Vec<(usize, usize)>],
        required_weight: usize,
        start_node: usize,
        end_node: usize,
    ) -> bool {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![false; adj.len()];
        queue.push_back(start_node);
        visited[start_node] = true;

        while let Some(current_node) = queue.pop_front() {
            for &(next_node, weight) in &adj[current_node] {
                if !visited[next_node] && weight >= required_weight {
                    visited[next_node] = true;
                    queue.push_back(next_node);
                }
            }
        }

        visited[end_node]
    }

    fn solution(start_node: usize, end_node: usize, edges: Vec<Vec<usize>>) -> usize {
        let n = edges
            .iter()
            .flat_map(|edge| edge.iter().take(2).cloned())
            .max()
            .unwrap_or(0);
        let mut adj = vec![vec![]; n + 1];
        let (mut lower_bound, mut upper_bound) = (usize::MAX, 0);

        edges.iter().for_each(|edge| {
            let (from, to, weight) = (edge[0], edge[1], edge[2]);
            adj[from].push((to, weight));
            adj[to].push((from, weight));
            lower_bound = lower_bound.min(weight);
            upper_bound = upper_bound.max(weight);
        });

        let mut optimal_weight = lower_bound;
        while lower_bound <= upper_bound {
            let mid_weight = (lower_bound + upper_bound) / 2;
            if bfs(&adj, mid_weight, start_node, end_node) {
                optimal_weight = mid_weight;
                lower_bound = mid_weight + 1;
            } else {
                upper_bound = mid_weight - 1;
            }
        }

        optimal_weight
    }
}
