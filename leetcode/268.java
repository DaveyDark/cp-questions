/*
Question:
Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
*/

// Approach:
// 1) Create a set of all the numbers in the array
// 2) Iterate through 0 to n and check if the number is present in the set
//     Return the number if it is not present in the set

class Solution {
  public int missingNumber(int[] nums) {
    Set<Integer> set = new HashSet<>();
    for(int i: nums) set.add(i);
    for(int i = 0; i <= nums.length; i++) {
      if(!set.contains(i)) return i;
    }
    return -1;
  }
}
