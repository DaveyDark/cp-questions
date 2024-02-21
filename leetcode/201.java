/*
Question:
Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.
*/ 

// Approach:
// We need find the common prefix of the binary representation of the two numbers.
// 1) We can right shift both the numbers until they are equal while keeping count of the number of shifts.
// 2) Then we again left shift the left number by the count of shifts and return the result.

class Solution {
  public int rangeBitwiseAnd(int left, int right) {
    int cnt = 0;
    while(left != right) {
      left = left >> 1;
      right = right >> 1;
      cnt++;
    }
    left = left << cnt;
    return left;
  }
}
