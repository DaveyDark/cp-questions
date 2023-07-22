/*
Question:
There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, 
and city b is connected directly with city c, then city a is connected indirectly with city c.

A province is a group of directly or indirectly connected cities and no other cities outside of the group.

You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, 
and isConnected[i][j] = 0 otherwise.

Return the total number of provinces.
*/

// Approach:
// We use a Vec of bools to store weather we have visited a node
// visit() is used to recursively visit all connected nodes of a node and mark them all as visited
// It works by looping over the connections of the given node and if a connection points to an unvisited node then we visit it
// 1) Make visited vector and a variable to track provinces
// 2) Loop through given nodes
//    If the node is already visited, skip that iteration
//    Otherwise call visit() on that node and increment provinces
// 3) After the loop ,return provinces

use std::collections::HashSet;

impl Solution {
    fn visit(visited: &mut Vec<bool>, connections: &Vec<Vec<i32>>, i: &usize) {
        visited[*i] = true;
        for j in 0..connections.len() {
            if connections[*i][j] != 1 || visited[j] {continue}
            Self::visit(visited,connections,&j);
        }
    }
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited: Vec<bool> = vec!(false; is_connected.len());
        let mut provinces = 0;
        for i in 0..is_connected.len() {
            if visited[i] {continue}
            Self::visit(&mut visited, &is_connected, &i);
            provinces += 1;
        }
        provinces
    }
}
