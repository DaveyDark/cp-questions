/*
Question:
Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.
*/

// Approach:
// We treat 0 as -1
// This transforms the problem into finding the longest subarray with sum = 0
// which can be done using a hashmap of prefix sum and index
// the prefix sum will be the key and the index will be the value
//
// We traverse the array and calculate the prefix sum then insert it into the hashmap
// If the prefix sum is already in the hashmap, the we have found a subarray with sum 0
// So we update the maxLen with the current length of the subarray

class Solution {
  public int findMaxLength(int[] nums) {
    HashMap<Integer, Integer> prefix = new HashMap<>();
    prefix.put(0,-1);
    int sum = 0;
    int maxLen = 0;
    
    for(int i = 0; i < nums.length; i++) {
      sum += nums[i] == 0 ? -1: 1;
      if(prefix.containsKey(sum))
        maxLen = Math.max(maxLen, i - prefix.get(sum));
      else
        prefix.put(sum, i);
    }

    return maxLen;
  }
}
