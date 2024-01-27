/*
Question:
For an integer array nums, an inverse pair is a pair of integers [i, j] 
where 0 <= i < j < nums.length and nums[i] > nums[j].

Given two integers n and k, 
return the number of different arrays consist of numbers from 1 to n 
such that there are exactly k inverse pairs. 
Since the answer can be huge, return it modulo 109 + 7.
*/

// Approach:
// A pattern is observed when manually calculating the result for the first few values of n and k.
// dp[i][j] is the sum of i values from the last row that satisfy the condition.
//
// 1) Make a dp table of size n x k+1 and initialize dp[0][0] = 1
// 2) Loop n times 
//      For each row, loop until j <= k or sum <= 0
//      Update the sum to hold the sum of the last i values from the previous row
//      Finally, set dp[i][j] = sum % MOD
// 3) Return dp[n-1][k]

class Solution {
  static final int MOD = 1_000_000_007;
  public int kInversePairs(int n, int k) {
    int[][] dp = new int[n][k+1];
    dp[0][0] = 1;
    for(int i = 1; i < n; i++) {
      long sum = 0;
      int start = 0, j = 0;
      while(j <= k) {
        sum += dp[i-1][j];
        if(j-start > i) sum -= dp[i-1][start++];
        if(sum <= 0) break;
        dp[i][j++] = (int)(sum % MOD);
      }
    }
    return dp[n-1][k];
  }
}
