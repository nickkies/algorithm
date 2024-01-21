#[cfg(test)]
mod tests {
    use log::debug;
    use std::{
        cmp::Ordering,
        collections::{BinaryHeap, HashMap, HashSet},
    };

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    struct Node {
        name: char,
        distance: usize,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distance.cmp(&self.distance)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    type Graph = HashMap<char, HashMap<char, usize>>;

    fn dijkstra(mut graph: Graph, (start, end): (char, char)) -> Option<usize> {
        let mut distances: HashMap<char, usize> = HashMap::new();
        let mut visited: HashSet<char> = HashSet::new();
        let mut priority_queue: BinaryHeap<Node> = BinaryHeap::new();

        distances.insert(start, 0);
        priority_queue.push(Node {
            name: start,
            distance: 0,
        });

        while let Some(Node { name, distance }) = priority_queue.pop() {
            if name == end {
                debug!("graph: {graph:?}");
                debug!("distances: {distances:?}");
                debug!("visited: {visited:?}");
                debug!("priority_queue: {priority_queue:?}");
                return Some(distance);
            }

            if visited.contains(&name) {
                continue;
            }

            visited.insert(name);

            if let Some(neighbors) = graph.remove(&name) {
                for (neighbor, weight) in neighbors {
                    let new_distance = distance + weight;
                    if let Some(&current_distance) = distances.get(&neighbor) {
                        if new_distance < current_distance {
                            distances.insert(neighbor, new_distance);
                            priority_queue.push(Node {
                                name: neighbor,
                                distance: new_distance,
                            });
                        }
                    } else {
                        distances.insert(neighbor, new_distance);
                        priority_queue.push(Node {
                            name: neighbor,
                            distance: new_distance,
                        });
                    }
                }
            }
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
