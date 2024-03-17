/*
Question:
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.
*/

// Approach:
// Calculate the product of all elements to the left of the current element and store it in prefix array
// Similarly calculate the product of all elements to the right of the current element and store it in suffix array
//
// Then, the result for index i will be prefix[i-1] * suffix[i+1] if i > 0 and i < n-1

class Solution {
  public int[] productExceptSelf(int[] nums) {
    int prefix[] = new int[nums.length];
    int suffix[] = new int[nums.length];

    int pfx = 1, sfx = 1;

    for(int i = 0; i < nums.length; i++) {
      pfx *= nums[i];
      sfx *= nums[nums.length - i - 1];
      suffix[nums.length - i - 1] = sfx;
      prefix[i] = pfx;
    }

    int res[] = new int[nums.length];

    for(int i = 0; i < nums.length; i++) {
      res[i] = 1;
      if(i > 0) res[i] *= prefix[i-1];
      if(i < nums.length - 1) res[i] *= suffix[i+1];
    }

    return res;
  }
}
