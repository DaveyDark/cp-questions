/*
Question:
There is an infrastructure of n cities with some number of roads connecting these cities. 
Each roads[i] = [ai, bi] indicates that there is a bidirectional road between cities ai and bi.

The network rank of two different cities is defined as the total number of directly connected roads to either city. 
If a road is directly connected to both cities, it is only counted once.

The maximal network rank of the infrastructure is the maximum network rank of all pairs of different cities.

Given the integer n and the array roads, return the maximal network rank of the entire infrastructure.
*/

// Approach:
// 1) Init an adjacency matrix adj which will be a HashMap mapping an i32(city) to a Vec<i32>(vector of adjacent cities) and populate it with keys for all cities
// 2) Loop through all the given roads and update the values in the adjacency matrix accordingly
// 3) Loop through all possible pairs of cities and calculate their rank by summing the lengths of their adjacency lists
//    Also subtract one from the rank if the cites are interconnecting to avoid adding duplicate connections
//    Keep track of and update the max rank found so far
// 4) Return the max recorded rank at the end

use std::collections::HashMap;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut adj = HashMap::new();
        for i in 0..n {
            adj.insert(i,vec!());
        }
        for road in &roads {
            adj.get_mut(&road[0]).unwrap().push(road[1]);
            adj.get_mut(&road[1]).unwrap().push(road[0]);
        }
        let mut max = 0;
        for c1 in 0..n {
            for c2 in 0..n {
                if c1 == c2 { continue }
                let mut rank = adj[&c1].len() + adj[&c2].len();
                if adj[&c2].contains(&c1) {
                    rank -= 1;
                }
                max = max.max(rank);
            }
        }
        max as i32
    }
}
