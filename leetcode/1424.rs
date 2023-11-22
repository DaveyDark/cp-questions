/*
Question:
Given a 2D integer array nums, return all elements of nums in diagonal order as shown in the below images.
*/

// Approach:
// Treat the grid as a graph and perform BFS on it
// 1) Make a queue for the BFS traversal and a vector to store the diagonal traversal
// 2) Push the initial (0,0) position to the queue
// 3) Loop until the queue becomes empty
//      Each iteration pop the coordinates (i,j) from the queue and check if there is an element at
//      those coordinates, if there is then push it to the diagonal vector and enqueue it's right
//      neightbor. Also enqueue it's lower neightbor if it is the first element of a row
// 4) Return the diagonal vector

use std::collections::VecDeque;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        let mut diag = vec!();
        queue.push_back((0,0));
        while !queue.is_empty() {
            let (i,j) = queue.pop_front().unwrap();
            if let Some(row) = nums.get(i) {
                if let Some(x) = row.get(j) {
                    diag.push(*x);
                    if j==0 { queue.push_back((i+1,j)) }
                    queue.push_back((i,j+1));
                }
            }
        }
        diag
    }
}
