/*
Question:
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
typically using all the original letters exactly once.
*/

// Approach:
// 1) If the length of the strings are not equal, return false
// 2) Create an array of size 26 to store the frequency of each character
// 3) Iterate through the strings and increment the frequency of each 
//    character in s and decrement the frequency of each character in t
// 4) If the frequency of any character is not 0, return false
// 5) Else return true

class Solution {
    public boolean isAnagram(String s, String t) {
        if(s.length() != t.length()) return false;
        int[] freq = new int[26];
        for(int i=0; i<s.length(); i++) {
            freq[s.charAt(i) - 'a'] += 1;
            freq[t.charAt(i) - 'a'] -= 1;
        }
        for(int x: freq) {
            if(x != 0) return false;
        }
        return true;
    }
}
