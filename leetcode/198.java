/*
Question:
You are a professional robber planning to rob houses along a street. 
Each house has a certain amount of money stashed, 
the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected 
and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, 
return the maximum amount of money you can rob tonight without alerting the police.
*/

// Approach:
// Iterate through the array and update the value at each index to be the max
// of the current value and the sum of the value two indices back and the
// last value. This works because you can't rob adjacent houses, so you
// can't rob both the house two indices back and the current house, so you
// can just take the max of the two options.
// Return the last element of the array.

class Solution {
  public int rob(int[] nums) {
    for (int i = 0; i < nums.length; i++) {
      if (i > 1)
        nums[i] = Math.max(nums[i - 2] + nums[i], nums[i - 1]);
      else if (i > 0)
        nums[i] = Math.max(nums[i - 1], nums[i]);
    }

    return nums[nums.length - 1];
  }
}
