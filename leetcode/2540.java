/*
Question:
Given two integer arrays nums1 and nums2, sorted in non-decreasing order, 
return the minimum integer common to both arrays. 
If there is no common integer amongst nums1 and nums2, return -1.

Note that an integer is said to be common to nums1 and nums2 if both arrays have at least one occurrence of that integer.
*/

// Approach:
// We can use two pointers to traverse both arrays and compare the elements at each index.
// If the elements are equal we return them
// Otherwise we increment the pointer of the array with the smaller element

class Solution {
  public int getCommon(int[] nums1, int[] nums2) {
    // Init pointers
    int i = 0, j = 0;
    
    // Loop until we reach the end of one of the arrays
    while(i < nums1.length && j < nums2.length) {
      // If the elements are equal we return them
      if(nums1[i] == nums2[j])
        return nums1[i];
      // If the first array element is smaller we increment its pointer
      else if(nums1[i] < nums2[j])
        i++;
      // Otherwise we increment the pointer of the second array
      else
        j++;
    }

    // If no match was found we return -1
    return -1;
  }
}
