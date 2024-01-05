/*
Question:
Given an integer array nums, return the length of the longest strictly increasing subsequence.
*/

// Approach:
// 1) Maintain a dp array of size n
//    dp[i] = length of longest increasing subsequence ending at i
// 2) Iterate over the array
//    For each element, find the max of dp[j] where j < i and nums[j] < nums[i]
//    If such a j exists, dp[i] = dp[j]+1
// 3) Find the max of dp[i] for all i
// 4) Return max+1

class Solution {
  public int lengthOfLIS(int[] nums) {
    int[] dp = new int[nums.length];
    for(int i = 1; i < nums.length; i++) {
      int j = 0;
      int max = i;
      for(j = i-1; j >= 0; j--) {
          if(nums[j] < nums[i] && dp[j] >= dp[max]) max = j;
      }
      if(max != i) dp[i] = dp[max]+1;
    }

    int max = 0;
    for(int i = 0; i < nums.length; i++) {
      max = Math.max(max, dp[i]);
    }

    return max+1;
  }
}
