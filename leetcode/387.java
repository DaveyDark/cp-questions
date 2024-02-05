/*
Question:
Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
*/

// Approach:
// 1) Create an array of size 26 to store the frequency of each character.
// 2) Iterate through the string and increment the frequency of each character.
// 3) Iterate through the string again and return the index of the first character with frequency 1.
// 4) If no such character is found, return -1.

class Solution {
  public int firstUniqChar(String s) {
    int[] freq = new int[26];
    for(char ch: s.toCharArray()) freq[ch - 'a']++;
    for(int i = 0; i < s.length(); i++) {
      if(freq[s.charAt(i) - 'a'] == 1) return i;
    }
    return -1;
  }
}
