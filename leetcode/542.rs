/*
Question:
Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
The distance between two adjacent cells is 1.
*/

// Approach:
// Using a queue, we enqueue all 0 elements and process them by checking their adjacent elements, then until all elements are processed. 
// Unset elements are set to -1
// 1) Make a queue to keep track of the nodes to process
// 2) First go through the matrix and enqueue all the 0 elements and mark elements of value > 0 as unset
// 3) Then loop until the queue is empty,
//    to process each element, check if any of the adjacent cells are unset and set them to the current cell + 1 and enqueue them
// 4) Return the modified matrix at the end

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let adjacent = [(0,-1),(-1,0),(0,1),(1,0)];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    queue.push_back((i as i32,j as i32));
                } else {
                    mat[i][j] = -1;
                }
            }
        }
        while let Some((i,j)) = queue.pop_front() {
            for (x,y) in &adjacent {
                if i+x < 0 || i+x >= mat.len() as i32 || j+y < 0 || j+y >= mat[0].len() as i32 { continue; }
                if mat[(i+x) as usize][(j+y) as usize] == -1 {
                    mat[(i+x) as usize][(j+y) as usize] = mat[i as usize][j as usize]+1;
                    queue.push_back((i+x,j+y));
                }
            }
        }
        mat
    }
}
