use std::collections::HashMap;

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let graph = create_graph(n, &connections);

    dfs(&graph)
}

#[inline]
fn create_graph(n: i32, connections: &Vec<Vec<i32>>) -> HashMap<i32, Vec<(i32, bool)>> {
    let mut graph = HashMap::from_iter((0..n).map(|i| (i, Vec::new())));

    for connection in connections {
        graph
            .get_mut(&connection[0])
            .unwrap()
            .push((connection[1], true));

        graph
            .get_mut(&connection[1])
            .unwrap()
            .push((connection[0], false));
    }

    graph
}

#[inline]
fn dfs(graph: &HashMap<i32, Vec<(i32, bool)>>) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    stack.push(0);

    let mut visited: HashMap<i32, bool> =
        HashMap::from_iter((0..graph.len()).map(|i| (i as i32, false)));

    let mut reorder = 0;

    while let Some(vertex) = stack.pop() {
        *visited.get_mut(&vertex).unwrap() = true;

        match graph.get(&vertex) {
            Some(edges) => edges.iter().for_each(|&e| {
                let (target, existing) = e;

                if let Some(false) = visited.get(&target) {
                    if existing {
                        reorder += 1;
                    }

                    stack.push(target);
                }
            }),
            None => {}
        }
    }

    reorder
}

#[cfg(test)]
mod tests {
    use super::min_reorder;

    #[test]
    fn ex1() {
        let result = min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
        );

        assert_eq!(result, 3);
    }

    #[test]
    fn ex2() {
        let result = min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]);

        assert_eq!(result, 2);
    }

    #[test]
    fn ex3() {
        let result = min_reorder(3, vec![vec![1, 0], vec![2, 0]]);

        assert_eq!(result, 0);
    }
}
