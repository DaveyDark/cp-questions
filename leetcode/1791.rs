/*
Question:
There is an undirected star graph consisting of n nodes labeled from 1 to n. 
A star graph is a graph where there is one center node and exactly n - 1 edges that connect the center node with every other node.
You are given a 2D integer array edges where each edges[i] = [ui, vi] indicates that there is an edge between the nodes ui and vi. 
Return the center of the given star graph.
*/

// Approach:
// The centeral node will be present in all edges, 
// so we just take the first two edges and return the common node

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<i32,i32> = HashMap::new();
        for edge in edges {
            *map.entry(edge[0]).or_insert(0) += 1;
            *map.entry(edge[1]).or_insert(0) += 1;
        }
        for (key,val) in map.iter() {
            if *val > 1 {return *key}
        }
        -1
    }
}
