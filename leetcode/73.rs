/*
Question:
Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.
You must do it in place.
*/

// Approach:
// 1) First we create a tuple that store two hashsets.
//    The first one will track the rows to be deleted
//    The second one will track the columns to be deleted
// 2) Then we go over the matrix and if we enounter any zero we add it's row and column to zeros tuple
//    Since we are using HashSets, even if we end up adding a duplicate, it will simply be ignored
// 3) Then we go over the rows in the matrix and if that row is in zeros.1 then we set it to all 0s
// 4) Then we go over the columns and if that column is in zeros.1 we set that column to all 0s

use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut zeros = (HashSet::new(),HashSet::new());
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    zeros.0.insert(i);
                    zeros.1.insert(j);
                }
            }
        }
        for i in 0..matrix.len() {
            if zeros.0.contains(&i) {
                for cell in matrix[i].iter_mut() { *cell = 0 }
            }
        }
        for j in 0..matrix[0].len() {
            if zeros.1.contains(&j) {
                for row in matrix.iter_mut() {row[j] = 0}
            }
        }
    }
}
