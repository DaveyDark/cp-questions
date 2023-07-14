/*
Question:
Given a string s and a string array dictionary, return the longest string in the dictionary that can be formed by deleting some of the given string characters. 
If there is more than one possible result, return the longest word with the smallest lexicographical order. 
If there is no possible result, return the empty string.
*/

// Approach:
// 1) Make a String to store the solution string, called max
// 2) Go though each of the Strings in the list and check if each one is a substring of the given string
//    If it is, check if it's length is equal, and if it is then check if it's lexicographically lesser than max
//    Otherwise check if it's length is more than max
//    If either of these are true, set max to the current string
// 3) Return max after the loop

class Solution {
    public boolean isSubsequence(String st, String sub) {
        for(int i=0, j=0; i<st.length(); i++) {
            if(st.charAt(i) == sub.charAt(j))j++;
            if(j==sub.length()) return true;
        }
        return false;
    }
    public String findLongestWord(String s, List<String> dictionary) {
        String max = "";
        for(String st: dictionary) {
            if(this.isSubsequence(s,st)){
                if(st.length() == max.length() && st.compareTo(max) < 0) max = st;
                else if(st.length() > max.length())max = st;
            }
        }
        return max;
    }
}
