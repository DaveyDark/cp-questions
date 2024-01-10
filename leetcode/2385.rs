/*
Question:
You are given the root of a binary tree with unique values,and an integer start.
At minute 0, an infection starts from the node with value start.

Each minute, a node becomes infected if:
    The node is currently uninfected.
    The node is adjacent to an infected node.

Return the number of minutes needed for the entire tree to be infected.
*/

// Approach:
// 1) Construct a graph from the tree as adjacency list
// 2) Perform a BFS from the start node and return the depth of the last node visited

use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    fn construct_graph(root: Option<Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
        if let Some(root_ref) = root {
            let root = root_ref.borrow();
            if let Some(left) = root.left.clone() {
                let l = left.borrow();
                graph.entry(root.val).or_insert(Vec::new()).push(l.val);
                graph.entry(l.val).or_insert(Vec::new()).push(root.val);
                Self::construct_graph(Some(left.clone()), graph);
            }
            if let Some(right) = root.right.clone() {
                let r = right.borrow();
                graph.entry(root.val).or_insert(Vec::new()).push(r.val);
                graph.entry(r.val).or_insert(Vec::new()).push(root.val);
                Self::construct_graph(Some(right.clone()), graph);
            }
        }
    }

    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut graph = HashMap::new();
        Self::construct_graph(root, &mut graph);

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        visited.insert(start);
        queue.push_back((start, 0));
        let (mut node, mut d) = (0, 0);
        while (!queue.is_empty()) {
            (node, d) = queue.pop_front().unwrap();
            for &nb in graph.get(&node).unwrap_or(&vec![]) {
                if visited.insert(nb) {
                    queue.push_back((nb, d + 1))
                }
            }
        }

        d
    }
}
