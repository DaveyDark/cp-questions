/*
Question:
You are given two strings of the same length s and t. 
In one step you can choose any character of t and replace it with another character.

Return the minimum number of steps to make t an anagram of s.

An Anagram of a string is a string that contains the same characters with a different (or the same) ordering.
*/

// Approach:
// 1) Create a freq array of size 26
// 2) Iterate over s and t and increment the freq of current char of s 
//    and decrement the freq of current char of t
// 3) Sum up the positive values in freq array
// 4) Return the sum

import java.util.*;

class Solution {
  public int minSteps(String s, String t) {
    int[] freq = new int[26];
    for(int i = 0; i < s.length(); i++) {
      freq[s.charAt(i) - 'a']++;
      freq[t.charAt(i) - 'a']--;
    }

    int sum = 0;
    for(int v: freq) {
      if(v > 0)sum += v;
    }
    return sum;
  }
}
