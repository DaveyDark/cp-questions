/*
Question:
Given a 2D integer array matrix, return the transpose of matrix.

The transpose of a matrix is the matrix flipped over its main diagonal,
switching the matrix's row and column indices.
*/

// Approach:
// 1) Create a new matrix with the dimensions of the transposed matrix
// 2) Iterate through each row of the original matrix and push each element into each row of the new matrix
// 3) Return the new matrix

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trans = vec![vec![]; matrix[0].len()];
        matrix
            .iter()
            .for_each(|row| row.iter().enumerate().for_each(|(i, n)| trans[i].push(*n)));
        trans
    }
}
