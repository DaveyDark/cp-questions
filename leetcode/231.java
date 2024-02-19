/*
Question:
Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2x.
*/

// Approach:
// 1) If n is less than or equal to 0, return false
// 2) If n is power of 2, then n & (n-1) will be 0
//     So, return (n & (n-1)) == 0

class Solution {
  public boolean isPowerOfTwo(int n) {
    if(n <= 0) return false;
    return (n & (n-1)) == 0;
  }
}
