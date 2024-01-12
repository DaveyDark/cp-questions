/*
Question:
You are given a string s of even length. 
Split this string into two halves of equal lengths, 
and let a be the first half and b be the second half.

Two strings are alike if they have the same number of vowels 
('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). 
Notice that s contains uppercase and lowercase letters.

Return true if a and b are alike. Otherwise, return false.
*/

// Approach:
// 1) Make two pointers, one at the start and one at the end
// 2) Loop until the two pointers meet
//    If the character at the left pointer is a vowel, increment the left count
//    If the character at the right pointer is a vowel, increment the right count
// 3) Return if the two counts are equal

class Solution {
  boolean isVowel(char c) {
    c = Character.toLowerCase(c);
    return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
  }
  public boolean halvesAreAlike(String s) {
    int l = 0, r = s.length()-1;
    int lc = 0, rc = 0;
    while(l < r) {
      if(isVowel(s.charAt(l))) lc++;
      if(isVowel(s.charAt(r))) rc++;
      l++;
      r--;
    }

    return lc == rc;
  }
}
