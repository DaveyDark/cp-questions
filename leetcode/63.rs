/*
Question:
You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). 
The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.

Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The testcases are generated so that the answer will be less than or equal to 2 * 109.
*/

// Approach:
// 1) If the start point is an obstacle, return 0 because we can't go anywhere
// 2) Make a 2d array same size as the grid for dp. dp[i][j] will represent the number of unique paths leading to grid[i][j]
//    Init the top left cell of dp to 1 since we start there
// 3) Loop through the cells of the grid
//    If current cell is an obstacle, we skip it
//    Otherwise, we check if the last row exists and if it does then we add the upper element's value to the current element
//    Similarly, if the last column exists, we add the left element's value to the current element
// 4) After the loop we can just return the bottom right corner of dp since that is the destination

impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {return 0}
        let mut dp = vec!(vec!(0; grid[0].len()); grid.len());
        dp[0][0] = 1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {continue}
                if i > 0 { dp[i][j] += dp[i-1][j] } 
                if j > 0 { dp[i][j] += dp[i][j-1] }
            }
        }
        dp[grid.len()-1][grid[0].len()-1]
    }
}
