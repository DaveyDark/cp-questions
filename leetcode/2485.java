/*
Question:
Given a positive integer n, find the pivot integer x such that:
    The sum of all elements between 1 and x inclusively equals the sum of all elements between x and n inclusively.

Return the pivot integer x. If no such integer exists, return -1. 
It is guaranteed that there will be at most one pivot index for the given input.
*/

// Approach
// We can use two pointers and prefix/suffix sums in order to find the pivot integer
// Initialize left and right pointers to 0 and n
// Initialize left and right sums to 0
// Loop until left and right pointers meet and update left and right sums
// If both sums are equal, return left or right
// If no pivot integer is found, return -1

class Solution {
  public int pivotInteger(int n) {
    // Handle edge case
    if(n==1)
      return 1;

    //Initialize left and right pointers and left and right sums
    int left = 0;
    int right = n;
    int lsum = 0;
    int rsum = 0;

    // Loop until left and right pointers meet
    while(left <= right) {
      // If left sum is smaller, move left pointer to the right and update left sum
      if(lsum < rsum)
        lsum += left++;
      // If right sum is smaller, move right pointer to the left and update right sum
      else if(lsum > rsum)
        rsum += right--;
      // Otherwise, if left and right sums are equal, return left or right
      else if(left == right)
        return left;
      // Otherwise, move left pointer to the right and update left sum
      else
        lsum += left++;
    }

    // If no pivot integer is found, return -1
    return -1;
  }
}
