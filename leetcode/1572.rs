/*
Question:
Given a square matrix mat, return the sum of the matrix diagonals.
Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.
*/

// Approach:
// Matrices with odd length have intersecting diagonals containing a common element at len/2
// Threfore we just need to sum both diagonals and subtract the element at len/2 if it's an odd length matrix

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        for i in 0..mat.len() { sum += mat[i][i] + mat[i][mat.len()-1-i]; }
        if mat.len()%2==0 {sum} else {sum - mat[mat.len()/2][mat.len()/2]}
    }
}
