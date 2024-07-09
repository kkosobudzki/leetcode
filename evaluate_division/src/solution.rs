use std::collections::HashMap;

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let graph = create_graph(&equations, &values);

    calculate(&graph, &queries)
}

#[inline]
fn create_graph(
    equations: &Vec<Vec<String>>,
    values: &Vec<f64>,
) -> HashMap<String, Vec<(String, f64)>> {
    // guaranteed by description
    assert!(equations.len() == values.len());

    let mut graph = HashMap::new();

    for i in 0..equations.len() {
        graph
            .entry(equations[i][0].clone())
            .or_insert(Vec::new())
            .push((equations[i][1].clone(), values[i]));

        graph
            .entry(equations[i][1].clone())
            .or_insert(Vec::new())
            .push((equations[i][0].clone(), 1.0 / values[i]));
    }

    graph
}

#[inline]
fn calculate(graph: &HashMap<String, Vec<(String, f64)>>, queries: &Vec<Vec<String>>) -> Vec<f64> {
    let mut answers = vec![-1.0; queries.len()];

    for i in 0..queries.len() {
        let q = &queries[i];

        if graph.get(&q[0]).is_some() && graph.get(&q[1]).is_some() {
            answers[i] = dfs(graph, q[0].clone(), q[1].clone());
        }
    }

    answers
}

#[inline]
fn dfs(graph: &HashMap<String, Vec<(String, f64)>>, source: String, target: String) -> f64 {
    let mut stack = Vec::new();
    stack.push((source.clone(), 1.0));

    let mut visited: HashMap<String, bool> = HashMap::new();

    // println!(
    //     "dfs => source: {source}, target: {target}, graph: {:?}",
    //     graph
    // );

    while let Some(head) = stack.pop() {
        let (vertex, answear) = head;

        // println!("dfs => vertex: {vertex}, answear: {answear}");

        if vertex == target {
            return answear;
        }

        visited.insert(vertex.clone(), true);

        graph.get(&vertex).unwrap().iter().for_each(|e| {
            // println!("dfs => edge: {:?}, visited: {:?}", e, visited);

            let (v, result) = e;

            // println!(
            //     "dfs => v: {v}, result: {result}, visited: {:?}",
            //     visited.get(v)
            // );

            if visited.get(v).is_none() {
                // println!("dfs => putting on stack v: {v}");

                stack.push((v.clone(), answear * result));
            }
        });
    }

    -1.0
}

#[cfg(test)]
mod tests {
    use super::calc_equation;

    #[test]
    fn ex1() {
        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("e")],
            vec![String::from("a"), String::from("a")],
            vec![String::from("x"), String::from("x")],
        ];

        let result = calc_equation(equations, values, queries);

        assert_eq!(result, vec![6.0, 0.5, -1.0, 1.0, -1.0]);
    }

    #[test]
    fn ex2() {
        let equations = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("c")],
            vec![String::from("bc"), String::from("cd")],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec![String::from("a"), String::from("c")],
            vec![String::from("c"), String::from("b")],
            vec![String::from("bc"), String::from("cd")],
            vec![String::from("cd"), String::from("bc")],
        ];

        let result = calc_equation(equations, values, queries);

        assert_eq!(result, vec![3.75, 0.4, 5.0, 0.2]);
    }

    #[test]
    fn ex3() {
        let equations = vec![vec![String::from("a"), String::from("b")]];
        let values = vec![0.5];
        let queries = vec![
            vec![String::from("a"), String::from("b")],
            vec![String::from("b"), String::from("a")],
            vec![String::from("a"), String::from("c")],
            vec![String::from("x"), String::from("y")],
        ];

        let result = calc_equation(equations, values, queries);

        assert_eq!(result, vec![0.5, 2.0, -1.0, -1.0]);
    }
}
