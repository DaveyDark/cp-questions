/*
Question:
You are given a 0-indexed m x n binary matrix grid.

A 0-indexed m x n difference matrix diff is created with the following procedure:
    Let the number of ones in the ith row be onesRowi.
    Let the number of ones in the jth column be onesColj.
    Let the number of zeros in the ith row be zerosRowi.
    Let the number of zeros in the jth column be zerosColj.
    diff[i][j] = onesRowi + onesColj - zerosRowi - zerosColj

Return the difference matrix diff.
*/

// Approach:
// For each cell, the value is the sum of the row and column difference.
// 1) Iterate over each row and calculate the difference between the
//    number of 1's and 0's in each row.
// 2) Iterate over each column and calculate the difference between the
//    number of 1's and 0's for each column.
// 3) Iterate over each cell and set the value to the sum of the row and
//    column difference.
// 4) Return the grid.

impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut row_diff: Vec<i32> = grid
            .iter()
            .map(|r| {
                let ones = r.iter().filter(|&x| *x == 1).count() as i32;
                ones - (n - ones)
            })
            .collect();
        let mut col_diff: Vec<i32> = (0..n)
            .map(|i| {
                let ones = grid
                    .iter()
                    .map(|r| r[i as usize])
                    .filter(|&x| x == 1)
                    .count() as i32;
                ones - (m - ones)
            })
            .collect();

        for i in 0..m as usize {
            for j in 0..n as usize {
                grid[i][j] = row_diff[i] + col_diff[j];
            }
        }
        grid
    }
}
