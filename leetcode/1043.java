/*
Question:
Given an integer array arr, partition the array into (contiguous) subarrays of length at most k. 
After partitioning, each subarray has their values changed to become the maximum value of that subarray.

Return the largest sum of the given array after partitioning. 
Test cases are generated so that the answer fits in a 32-bit integer.
*/

// Approach:
// 1) Create a dp array of size N and initialize it with -1
// 2) Create a recursive function maxSum() to calculate the maximum sum
// 3) In maxSum(), if start >= N, return 0
// 4) If dp[start] != -1, return dp[start]
// 5) Initialize currMax and ans to 0 and Calculate the end point of the current subarray
//    Iterate from start to end and calculate the maximum value of the subarray
//    Then calculate the ans by taking the maximum of the current ans and the maximum value of the subarray
//    multiplied by the length of the subarray and the maximum sum of the remaining array
// 6) Return dp[start] = ans
// 7) In the main function, return maxSum(arr, k, dp, 0)

class Solution {
  private int maxSum(int[] arr, int k, int[] dp, int start) {
    int N = arr.length;

    if (start >= N) {
      return 0;
    }

    if (dp[start] != -1) {
      return dp[start];
    }

    int currMax = 0, ans = 0;
    int end = Math.min(N, start + k);
    for (int i = start; i < end; i++) {
      currMax = Math.max(currMax, arr[i]);
      ans = Math.max(ans, currMax * (i - start + 1) + maxSum(arr, k, dp, i + 1));
    }

    return dp[start] = ans;
  }

  public int maxSumAfterPartitioning(int[] arr, int k) {
    int[] dp = new int[arr.length];
    Arrays.fill(dp, -1);

    return maxSum(arr, k, dp, 0);
  }
}
