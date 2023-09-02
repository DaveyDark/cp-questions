/*
Question:
There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). 
The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). 
The robot can only move either down or right at any point in time.

Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.
*/

// Approach:
// 1) Make a dp vector the size of the grid filled with 0s
// 2) Init the first cell with 1
// 2) Loop though every cell in the grid, and set the value of every cell to the sum of the value
//    in the top and the left cell
// 3) Return the value in the bottom right cell

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec!(vec!(0; n as usize); m as usize);
        dp[0][0] = 1;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if i > 0 {
                    dp[i][j] += dp[i-1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j-1];
                }
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}
