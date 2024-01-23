#[cfg(test)]
mod tests {
    use log::debug;

    struct Graph {
        vertices: Vec<char>,
        edges: Vec<(usize, char, char)>,
    }

    impl Graph {
        fn new() -> Self {
            Self {
                vertices: vec!['A', 'B', 'C', 'D', 'E', 'F', 'G'],
                edges: vec![
                    (7, 'A', 'B'),
                    (5, 'A', 'D'),
                    (7, 'B', 'A'),
                    (8, 'B', 'C'),
                    (9, 'B', 'D'),
                    (7, 'B', 'E'),
                    (8, 'C', 'B'),
                    (5, 'C', 'E'),
                    (5, 'D', 'A'),
                    (9, 'D', 'B'),
                    (7, 'D', 'E'),
                    (6, 'D', 'F'),
                    (7, 'E', 'B'),
                    (5, 'E', 'C'),
                    (7, 'E', 'D'),
                    (8, 'E', 'F'),
                    (9, 'E', 'G'),
                    (6, 'F', 'D'),
                    (8, 'F', 'E'),
                    (11, 'F', 'G'),
                    (9, 'G', 'E'),
                    (11, 'G', 'F'),
                ],
            }
        }

        fn kruskal(&self) -> Option<usize> {
            let len = self.vertices.len();
            let mut sum = 0;
            let mut remain = len;
            let mut parents: Vec<usize> = (0..len).collect();
            let mut rank: Vec<usize> = vec![0; len];

            let mut edges = self.edges.clone();
            edges.sort_by_key(|&(weight, _, _)| weight);

            edges.iter().for_each(|(weight, start, end)| {
                let (mut start_idx, mut end_idx) = (0, 0);
                self.vertices.iter().enumerate().for_each(|(idx, vertice)| {
                    if vertice == start {
                        start_idx = idx;
                    } else if vertice == end {
                        end_idx = idx;
                    }
                });

                if Self::find(&mut parents, start_idx) != Self::find(&mut parents, end_idx) {
                    let root1 = Self::find(&mut parents, start_idx);
                    let root2 = Self::find(&mut parents, end_idx);

                    if root1 != root2 {
                        let rank1 = rank[root1];
                        let rank2 = rank[root2];

                        if rank1 > rank2 {
                            parents[root2] = root1;
                        } else {
                            parents[root1] = root2;

                            if rank1 == rank2 {
                                rank[root2] += 1;
                            }
                        }
                    }

                    debug!("parents: {:?}, rank: {:?}", parents, rank);
                    debug!("weight: {}, start: {}, end: {}", weight, start, end);

                    sum += weight;
                    remain -= 1;
                }
            });

            if remain == 1 {
                Some(sum)
            } else {
                None
            }
        }

        fn find(parents: &mut Vec<usize>, node: usize) -> usize {
            debug!(
                "[find] parent[node]: {}, node: {}, {}",
                parents[node],
                node,
                parents[node] != node
            );

            if parents[node] != node {
                parents[node] = Self::find(parents, parents[node]);
            }
            parents[node]
        }
    }

    #[test]
    fn test_kruskal() {
        let graph = Graph::new();

        let answer = graph.kruskal();

        assert!(answer.is_some());
        assert_eq!(answer.unwrap(), 39);
    }
}
