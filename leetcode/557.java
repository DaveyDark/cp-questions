/*
Question:
Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.
*/

// Approach:
// 1) We split the string by spaces
// 2) Then for each split string we reverse it
// 3) Then we add all individual words and return the trimmed string(to remove tailing spaces)

class Solution {
    public String reverse(String str){
        char[] s = str.toCharArray();
        for(int i=0; i<s.length/2; i++){
            char tmp = s[s.length - 1 - i];
            s[s.length -1 - i] = s[i];
            s[i] = tmp;
        }
        return new String(s);
    }
    public String reverseWords(String s) {
        String[] words = s.split(" ");
        s = "";
        for(String word: words){
            s += reverse(word) + " ";
        }
        return s.trim();
    }
}
