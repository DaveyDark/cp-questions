/*
Question:
Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".
*/

// Approach:
// 1) We make len to store the length of the longest prefix, and init it to the length of the first word
// 2) Then we go over the rest of the words and for each word, 
//    We see how many characters are requal between it and the first word
//    Then we set len to the minimum number of matched chars
// 3) At the end we return the substring of the first word upto len length

class Solution {
    public String longestCommonPrefix(String[] strs) {
        int len = strs[0].length();
        for(int i=1; i<strs.length; i++) {
            int ctr = 0;
            for(int j=0; j<strs[i].length() && j<strs[0].length(); j++) {
                if(strs[i].charAt(j) != strs[0].charAt(j))break;
                ctr++;
            }
            len = Math.min(len,ctr);
        }
        return strs[0].substring(0,len);
    }
}
