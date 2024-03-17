/*
Question:
Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.

A subarray is a contiguous part of the array.
*/

// Approach:
// We create an array and store the number of subarrays with a given sum.
// The prefix sum is the key and the value is the number of subarrays with that sum.
//
// Then the toal count can be calculated by summing (sums[i] * sums[i - goal]) for all i >= goal.
//
// If the goal is 0, we instead need to calculate the sum of natural numbers upto f for all f > 1.(frequency)

class Solution {
  public int numSubarraysWithSum(int[] nums, int goal) {
    int sums[] = new int[nums.length+1];
    sums[0] = 1;
    int sum = 0;
    for (int i = 0; i < nums.length; i++) {
      sum += nums[i];
      sums[sum]++;
    }

    int cnt = 0;

    if(goal == 0){
      for(int s: sums) {
        for(int i = 1; i < s; i++) cnt += i;
      }
      return cnt;
    }

    for(int i = goal; i < sums.length; i++) {
      cnt += sums[i] * sums[i - goal];
    }

    return cnt;
  }
}
