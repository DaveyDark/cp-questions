/*
Question:
There is an m x n grid with a ball. 
The ball is initially at the position [startRow, startColumn]. 
You are allowed to move the ball to one of the four adjacent cells in the grid 
(possibly out of the grid crossing the grid boundary). 
You can apply at most maxMove moves to the ball.

Given the five integers m, n, maxMove, startRow, startColumn, 
return the number of paths to move the ball out of the grid boundary. 
Since the answer can be very large, return it modulo 109 + 7.
*/

// Approach:
// 1) Make a 2D dp array of size m*n. dp[i][j] represents the number of ways to reach i,j in maxMove moves.
// 2) Initialize dp[startRow][startColumn] = 1
// 3) For each move, 
//      Make a new dp array _dp
//      For each non zero dp[i][j] in dp, 
//        Explore all the 4 neighbours of i,j
//          If the neighbour is out of bounds, increment ways by dp[i][j]
//          Otherwise, add dp[i][j] to _dp[nx][ny]
//      Set dp = _dp
// 4) Return ways

class Solution {
  static final int nb[][] = { { -1, 0 }, { 1, 0 }, { 0, 1 }, { 0, -1 } };
  static final int MOD = 1_000_000_007;

  public int findPaths(int m, int n, int maxMove, int startRow, int startColumn) {
    int dp[][] = new int[m][n];
    dp[startRow][startColumn] = 1;
    long ways = 0;
    for (int moves = maxMove; moves > 0; moves--) {
      int _dp[][] = new int[m][n];
      for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
          if (dp[i][j] > 0) {
            for (int[] N : nb) {
              int nx = i + N[0];
              int ny = j + N[1];
              if (nx < 0 || ny < 0 || nx == m || ny == n)
                ways = (ways + dp[i][j]) % MOD;
              else
                _dp[nx][ny] = (_dp[nx][ny] + dp[i][j]) % MOD;
            }
          }
        }
      }
      dp = _dp;
    }

    return (int) ways;
  }
}
