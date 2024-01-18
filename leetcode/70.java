/*
Question:
You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

*/

// Approach:
// 1) If n is 1, return 1
// 2) Create a dp array of size n
// 3) Initialize dp[0] = 1, dp[1] = 2
// 4) For the rest of the array, dp[i] = dp[i-1] + dp[i-2]
// 5) Return dp[n-1]

class Solution {
  public int climbStairs(int n) {
    if(n == 1) return 1;
    int[] dp = new int[n];
    dp[0] = 1;
    dp[1] = 2;
    for(int i = 2; i < n; i++) {
      dp[i] = dp[i-1] + dp[i-2];
    }
    return dp[n-1];
  }
}
