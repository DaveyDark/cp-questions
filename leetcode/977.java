/*
Question:
Given an integer array nums sorted in non-decreasing order, 
return an array of the squares of each number sorted in non-decreasing order.
*/

// Approach:
// We can square each element and then arrange the elements by taking two pointers on both ends and then
// adding the greater element to the result array and updating the pointers. 
// This will create a descending order of the elements. 
// We can then reverse the array to get the ascending order.

class Solution {
  public int[] sortedSquares(int[] nums) {
    int l = nums.length;

    // Square each element
    for(int i = 0; i < l; i++) {
      nums[i] = nums[i] * nums[i];
    }

    // Array to store sorted squares
    int[] res = new int[l];

    // Two pointers to traverse the array
    int left = 0;
    int right = l-1;

    // Pointer to store the position to insert the next element
    int ptr = 0;

    // Loop until the two pointers meet
    while(left <= right) {
      // Add the greater element to the result array and update the pointers
      if(nums[left] > nums[right]) 
        res[ptr++] = nums[left++];
      else 
        res[ptr++] = nums[right--];
    }

    // Reverse the array 
    for(int i = 0; i < l/2; i++) {
      int tmp = res[i];
      res[i] = res[l-i-1];
      res[l-i-1] = tmp;
    }

    return res;
  }
}
