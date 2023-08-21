/*
Question:
Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.
 */

// Approach:
// 1) Loop over lengths from 1 to s.length/2
// 2) Check if the current length can fully divide s.length
//    If it can then check if all substring of length i from the start to the end of the string are equal
//    Return true if they are
// 3) Otherwise if we reach outside the loop, return false

class Solution {
    public boolean repeatedSubstringPattern(String s) {
        for(int i=1; i<=s.length()/2; i++) {
            if(s.length()%i == 0) {
                boolean flag = true;
                for(int j=0; j<s.length()/i; j++) {
                    flag = flag && s.substring(j*i,(j+1)*i).equals(s.substring(0,i));
                }
                if(flag)return true;
            }
        }
        return false;
    }
}
