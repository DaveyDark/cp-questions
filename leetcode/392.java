/*
Question:
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) 
of the characters without disturbing the relative positions of the remaining characters. 
(i.e., "ace" is a subsequence of "abcde" while "aec" is not).
*/

// Approach:
// 1) Make a pointer j to traverse s
// 2) Loop through t using index i
//      If the current chat of t is equal to the current char of s, increment j
// 3) If j is euqal to the length of s, it is a subsequence

class Solution {
    public boolean isSubsequence(String s, String t) {
        int j = 0;
        for(int i=0; i<t.length() && j<s.length(); i++) {
            if(t.charAt(i) == s.charAt(j)) j++;
        }
        return j == s.length();
    }
}
