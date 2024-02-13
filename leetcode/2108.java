/*
Question:
Given an array of strings words, return the first palindromic string in the array. 
If there is no such string, return an empty string "".

A string is palindromic if it reads the same forward and backward.
*/

// Approach:
// 1) Iterate through the array of strings
// 2) For each string, check if it is a palindrome and return it if it is
// 3) If no palindrome is found, return an empty string

class Solution {
  boolean isPalindrome(String s) {
    for (int i = 0; i < s.length() / 2; i++) {
      if (s.charAt(i) != s.charAt(s.length() - 1 - i))
        return false;
    }
    return true;
  }

  public String firstPalindrome(String[] words) {
    for (String s : words) {
      if (isPalindrome(s))
        return s;
    }
    return "";
  }
}
