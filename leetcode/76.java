/*
Question:
Given two strings s and t of lengths m and n respectively, 
return the minimum window substring of s such that every character in t (including duplicates) 
is included in the window. If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.
*/

// Approach:
// Since the strings contain both uppercase and lowercase characters, we use a utility function index()
// to map each letter to an index in a 52 length array. 
// 1) Initialize two pointers, l and r to 0 and t.length() respectively.
// 2) Calculate the frequency of characters in t and store it in target[]
// 3) Caclulate the frequency of first t.length() characters in s and store it in freq[]
// 4) Initialize min = Integer.MAX_VALUE and minStr = ""
// 5) Loop While r <= s.length()
//     While the window includes all the characters of t,
//      Set min and minStr if the current window is smaller than min
//      Remove the character at l from the window and increment l
//     If r == s.length() break
//     Add the character at r to the window and increment r
// 6) Return minStr

class Solution {
  int index(char c) {
    return Character.isLowerCase(c) ? c - 'a' : c - 'A' + 26;
  }
  int[] countFreq(String s) {
    int[] freq = new int[52];
    for(char c: s.toCharArray()) {
      freq[index(c)]++;
    }
    return freq;
  }
  boolean includes(int[] f1, int[] f2) {
    for(int i = 0; i < f1.length; i++) {
      if(f1[i] < f2[i]) return false;
    }
    return true;
  }
  public String minWindow(String s, String t) {
    int l = 0, r = t.length();
    int[] freq = countFreq(s.substring(0, Math.min(r, s.length())));
    int[] target = countFreq(t);
    int min = Integer.MAX_VALUE;
    String minStr = "";
    
    while(r <= s.length()) {
      while(includes(freq, target) && l < r)  {
        if(min > r-l) {
          min = r-l;
          minStr = s.substring(l,r);
        }
        freq[index(s.charAt(l++))]--;
      }
      if(r == s.length()) break;
      freq[index(s.charAt(r++))]++;
    }

    return minStr;
  }
}
