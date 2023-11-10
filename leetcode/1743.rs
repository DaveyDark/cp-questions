/*
Question:
There is an integer array nums that consists of n unique elements, but you have forgotten it. 
However, you do remember every pair of adjacent elements in nums.

You are given a 2D integer array adjacentPairs of size n - 1 where each adjacentPairs[i] = [ui, vi] 
indicates that the elements ui and vi are adjacent in nums.

It is guaranteed that every adjacent pair of elements nums[i] and nums[i+1] will exist in adjacentPairs, 
either as [nums[i], nums[i+1]] or [nums[i+1], nums[i]]. The pairs can appear in any order.

Return the original array nums. If there are multiple solutions, return any of them.
*/

// Approach:
// 1) Make a hashmap and loop over the given pairs and contruct an adjacency list for the graph
// 2) Make a vector to store the ans, a HashSet to store visited nodes and a stack to store nodes
//    to perform DFS
// 3) Loop over the map to find a leaf node of the graph and push it into the stack
// 4) Until the stack is empty, pop an element and push the adjacent vertices of that element into
//    the stack and also push the element into the ans vector.
// 5) Return the constructed answer vector.

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();

        for pair in &adjacent_pairs {
            map.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);
            map.entry(pair[1]).or_insert(Vec::new()).push(pair[0]);
        }

        let mut arr = vec!();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        for (key,val) in map.iter() {
            if val.len() == 1 {
                stack.push(*key);
                break;
            }
        }

        while let Some(curr) = stack.pop() {
            if visited.insert(curr) {
                arr.push(curr);
                stack.append(map.get_mut(&curr).unwrap());
            }
        }

        arr
    }
}
