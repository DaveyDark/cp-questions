/*
Question:
Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
The distance between two adjacent cells is 1.
*/

// Approach:
// Using a queue, we enqueue all elements of a certain value, starting from 0, until all elements are processed. 
// Unprocessed elements are set to -1
// To process an element we just set the adjacent unprocessed elements to the value of the current element + 1
// 1) We make a queue and a var to track the current processing value and then we start the loop
// 2) We first go through the matrix and enqueue all elements of current value and mark elements of value > current as unprocessed
// 3) If no elements are enqueued then we have no elements to process so we break out of the loop
// 4) Then we go through the queue and process each element in the queue
// 5) Then we increment the current at the end of iteration
// 6) We return the modified matrix at the end

use std::collections::VecDeque;

impl Solution {
    pub fn set_adjacent(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) {
        let adjacent = [(0,-1),(-1,0),(0,1),(1,0)];
        for (x,y) in adjacent {
            if i+x < 0 || i+x >= grid.len() as i32 || j+y < 0 || j+y >= grid[0].len() as i32 {continue;}
            if grid[(i+x) as usize][(j+y) as usize] == -1 {
                grid[(i+x) as usize][(j+y) as usize] = grid[i as usize][j as usize] + 1;
            }
        }
    }
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat.clone();
        let mut queue = VecDeque::new();

        let mut current = 0;
        loop {
            for i in 0..mat.len() {
                for j in 0..mat[0].len() {
                    if mat[i][j] == current {
                        queue.push_back((i,j));
                    } else if mat[i][j] > current {
                        mat[i][j] = -1;
                    }
                }
            }
            if queue.is_empty() {break;}
            while !queue.is_empty() {
                let (i,j) = queue.pop_front().unwrap();
                Solution::set_adjacent(&mut mat, i as i32, j as i32);
            }
            current += 1;
        }
        mat
    }
}
