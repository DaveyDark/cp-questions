/*
Question:
You are given a rows x cols matrix grid representing a field of cherries 
where grid[i][j] represents the number of cherries that you can collect from the (i, j) cell.

You have two robots that can collect cherries for you:
    Robot #1 is located at the top-left corner (0, 0), and
    Robot #2 is located at the top-right corner (0, cols - 1).

Return the maximum number of cherries collection using both robots by following the rules below:
    From a cell (i, j), robots can move to cell (i + 1, j - 1), (i + 1, j), or (i + 1, j + 1).
    When any robot passes through a cell, It picks up all cherries, and the cell becomes an empty cell.
    When both robots stay in the same cell, only one takes the cherries.
    Both robots cannot move outside of the grid at any moment.
    Both robots should reach the bottom row in grid.
*/

// Approach:
// 1) Make a 3D dp array of size 2 x n x n
//    i -> row
//    j -> column for first robot
//    k -> column for second robot
//    dp[i][j][k] -> maximum cherries that can be picked by both robots at ith row, jth column and kth column
//    Since we only need the current and previous rows, we can use 3D dp array of size 2 x n x n
//    Where i+1 % 2 will give us the previous row and i % 2 will give us the current row
// 2) Fill the dp array with -1
//    Initialize dp[0][0][n - 1] = grid[0][n - 1] + grid[0][0]
// 3) Iterate over all cells in the dp array and fill the dp array
//     For each cell, we have 9 possible moves for both robots
//     like choosing the j-1 and k-1 column for the robots etc etc
//     We will find the max of all these possible moves then store it in dp[i][j][k] if it is not -1
//     Also, update the ans with max of ans and dp[i][j][k]
// 4) Return ans

class Solution {
  public int cherryPickup(int[][] grid) {
    final int m = grid.length, n = grid[0].length;
    int[][][] dp = new int[2][n][n];
    for (int i = 0; i < 2; i++)
      for (int j = 0; j < n; j++)
        Arrays.fill(dp[i][j], -1);
    dp[0][0][n - 1] = grid[0][n - 1] + grid[0][0];
    int ans = 0;
    for (int i = 1; i < m; i++) {
      for (int j = 0; j < n; j++) {
        for (int k = j + 1; k < n; k++) {
          int max = -1;
          for (int x = -1; x < 2; x++) {
            for (int y = -1; y < 2; y++) {
              if (j + x < 0 || j + x >= n || k + y < 0 || k + y >= n)
                continue;
              max = Math.max(max, dp[(i + 1) % 2][j + x][k + y]);
            }
          }
          if (max != -1)
            dp[i % 2][j][k] = max + grid[i][j] + grid[i][k];
          ans = Math.max(ans, dp[i % 2][j][k]);
        }
      }
    }

    return ans;
  }
}
