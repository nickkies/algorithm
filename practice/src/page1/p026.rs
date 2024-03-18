#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Node {
        left_node: Option<char>,
        right_node: Option<char>,
    }

    // Pre-order tree traversal: Node -> Left -> Right
    fn preorder_traversal(node: &char, tree: &HashMap<char, Node>) -> String {
        let mut res = node.to_string();
        if let Some(left) = tree[node].left_node {
            res += &preorder_traversal(&left, tree);
        }
        if let Some(right) = tree[node].right_node {
            res += &preorder_traversal(&right, tree);
        }

        res
    }

    // In-order tree traversal: Left -> Node -> Right
    fn inorder_traversal(node: &char, tree: &HashMap<char, Node>) -> String {
        let mut res = String::new();
        if let Some(left) = tree[node].left_node {
            res += &inorder_traversal(&left, tree);
        }

        res.push(*node);

        if let Some(right) = tree[node].right_node {
            res += &inorder_traversal(&right, tree);
        }

        res
    }

    // Post-order tree traversal: Left -> Right -> Node
    fn postorder_traversal(node: &char, tree: &HashMap<char, Node>) -> String {
        let mut res = String::new();
        if let Some(left) = tree[node].left_node {
            res += &postorder_traversal(&left, tree);
        }
        if let Some(right) = tree[node].right_node {
            res += &postorder_traversal(&right, tree);
        }

        res.push(*node);

        res
    }

    fn solution(input: Vec<(char, char, char)>) -> Vec<String> {
        let mut tree: HashMap<char, Node> = HashMap::new();
        let start_node = 'A';

        input.into_iter().for_each(|(data, left, right)| {
            let left_node = if left != '.' { Some(left) } else { None };
            let right_node = if right != '.' { Some(right) } else { None };

            tree.insert(
                data,
                Node {
                    left_node,
                    right_node,
                },
            );
        });

        log::debug!("{tree:?}");

        vec![
            preorder_traversal(&start_node, &tree),
            inorder_traversal(&start_node, &tree),
            postorder_traversal(&start_node, &tree),
        ]
    }

    // 백준 1991
    #[test]
    fn 트리_순회() {
        let input = vec![
            ('A', 'B', 'C'),
            ('B', 'D', '.'),
            ('C', 'E', 'F'),
            ('E', '.', '.'),
            ('F', '.', 'G'),
            ('D', '.', '.'),
            ('G', '.', '.'),
        ];

        assert_eq!(
            solution(input),
            vec![
                "ABDCEFG".to_string(),
                "DBAECFG".to_string(),
                "DBEGFCA".to_string()
            ]
        )
    }
}
