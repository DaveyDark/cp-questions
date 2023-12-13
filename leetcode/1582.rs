/*
Question:
Given an m x n binary matrix mat, return the number of special positions in mat.

A position (i, j) is called special if mat[i][j] == 1 and all other elements in
row i and column j are 0 (rows and columns are 0-indexed).
*/

// Approach:
// 1) Create two arrays to store the count of 1's in each row and column
// 2) Iterate through the matrix and increment the count of 1's in each row and column
// 3) Iterate through the matrix again and check if the current element is 1
//      and the count of 1's in the row and column is 1. If it is, increment count
// 4) Return count

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut cnt = 0;
        let mut col_counts = vec![0; mat[0].len()];
        let mut row_counts = vec![0; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 {
                    row_counts[i] += 1;
                    col_counts[j] += 1;
                }
            }
        }
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && row_counts[i] == 1 && col_counts[j] == 1 {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}
