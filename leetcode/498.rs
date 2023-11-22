/*
Question:
Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.
*/

// Approach:
// 1) Make a vec res to store the traversal, and variables m and n to represent the length and width of
//    the matrix
// 2) Loop over numbers from 0 to m+n, where the curr will be the sum the indices in this diagonal
//    will make
//      If curr is even, we calculate the starting point for the diagonal using the given formula
//      and then move diagonally upright until we go out of bounds and push each element to res
//      
//      If curr is odd, we calculate the starting point for the diagonal using another formula and
//      then move diagonally downleft until we go out of bounds and push each element to res
// 3) Return the res vector

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec!();
        let m = mat.len()-1;
        let n = mat[0].len()-1;
        for curr in 0..=m+n {
            if curr%2 == 0 {
                let (mut i, mut j) = (curr.min(m),(curr as i32 - m as i32).max(0) as usize);
                while i <= m && j <= n {
                    res.push(mat[i][j]);
                    i -= 1;
                    j += 1;
                }
            } else {
                let (mut i, mut j) = ((curr as i32 - n as i32).max(0) as usize,curr.min(n));
                while i <= m && j <= n {
                    res.push(mat[i][j]);
                    i += 1;
                    j -= 1;
                }
            }
        }
        res
    }
}
