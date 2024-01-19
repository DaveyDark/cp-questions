/*
Question:
Given an n x n array of integers matrix, return the minimum sum of any falling path through matrix.

A falling path starts at any element in the first row and chooses the element in the next row that is either directly below or diagonally left/right.
Specifically, the next element from position (row, col) will be (row + 1, col - 1), (row + 1, col), or (row + 1, col + 1).
*/

// Approach:
// 1) Iterate over the matrix
// 2) For each element, find the minimum of the previous row
// 3) Add the minimum to the current element
// 4) Return the minimum of the last row

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        for i in 1..matrix.len() {
            for j in 0..matrix[0].len() {
                let mut last = i32::MAX;
                last = last.min(*matrix[i - 1].get(j).unwrap());
                if j > 0 {
                    last = last.min(*matrix[i - 1].get(j - 1).unwrap())
                }
                if j < matrix.len() - 1 {
                    last = last.min(*matrix[i - 1].get(j + 1).unwrap())
                }
                matrix[i][j] += last;
            }
        }

        *matrix[matrix.len() - 1].iter().min().unwrap()
    }
}
