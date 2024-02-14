/*
Question:
You are given a 0-indexed integer array nums of even length consisting of an equal number of positive and negative integers.

You should rearrange the elements of nums such that the modified array follows the given conditions:
    Every consecutive pair of integers have opposite signs.
    For all integers with the same sign, the order in which they were present in nums is preserved.
    The rearranged array begins with a positive integer.

Return the modified array after rearranging the elements to satisfy the aforementioned conditions.
*/

// Approach:
// 1) Make two arrays, one for positive numbers and one for negative numbers
// 2) Traverse the original array and fill the positive and negative arrays
// 3) Traverse the original array and fill the values from the positive and negative arrays
//    For even indices, fill from the positive array
//    For odd indices, fill from the negative array
// 4) Return the original array

class Solution {
  public int[] rearrangeArray(int[] nums) {
    final int l = nums.length;
    int[] p = new int[l/2];
    int[] n = new int[l/2];
    int pi = 0, ni = 0;
    for(int i = 0; i < l; i++) {
      if(nums[i] < 0) {
        n[ni++] = nums[i];
      } else {
        p[pi++] = nums[i];
      }
    }
    for(int i = 0; i < l; i++) {
      if(i%2 == 1) {
        nums[i] = n[i/2];
      } else {
        nums[i] = p[i/2];
      }
    }
    return nums;
  }
}
