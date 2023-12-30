/*
Questions:
You are given an array of variable pairs equations and an array of real numbers values,
where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i].
Each Ai or Bi is a string that represents a single variable.

You are also given some queries, where queries[j] = [Cj, Dj]
represents the jth query where you must find the answer for Cj / Dj = ?.

Return the answers to all queries. If a single answer cannot be determined, return -1.0.

Note: The input is always valid.
You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.

Note: The variables that do not occur in the list of equations are undefined,
so the answer cannot be determined for them.
*/

// Approach:
// 1) Build a directed graph as an adjacency list from the equations and values
// 2) For each query,
//    If either node is not in the graph, push -1.
//    If the two nodes are teh same, push 1.
//    Otherwise, run a BFS on the graph from the first to the second variable using search_graph().
// 3) In search_graph, we use a queue to run BFS on the graph. We also keep a visited set to avoid
//    cycles. If we find the destination node, we return the product of the weights from the source
//    to the destination, or -1 otherwise
// 4) Return the vector of the results of all queries

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
        for (eq, val) in equations.iter().zip(values.iter()) {
            graph.entry(&eq[0]).or_insert(vec![]).push((&eq[1], *val));
            graph
                .entry(&eq[1])
                .or_insert(vec![])
                .push((&eq[0], 1.0 / val));
        }
        let mut results = vec![];
        for q in &queries {
            if !graph.contains_key(q[0].as_str()) || !graph.contains_key(q[1].as_str()) {
                results.push(-1.0);
            } else if q[0] == q[1] {
                results.push(1.0);
            } else {
                results.push(Self::search_graph(&graph, &q[0], &q[1]));
            }
        }
        results
    }
    fn search_graph(graph: &HashMap<&str, Vec<(&str, f64)>>, src: &str, dest: &str) -> f64 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((src, 1.0));
        while let Some(node) = queue.pop_front() {
            if node.0 == dest {
                return node.1;
            }
            if visited.contains(node.0) {
                continue;
            }
            visited.insert(node.0.clone());
            for nb in graph.get(node.0).unwrap() {
                queue.push_back((nb.0, node.1 * nb.1));
            }
        }
        -1.0
    }
}
