/*
Question:
Write a function that reverses a string. The input string is given as an array of characters s.
You must do this by modifying the input array in-place with O(1) extra memory.
*/

// Approach:
// We loop the first half of the array and swap the ith character with the (length - i)th character

class Solution {
    public void reverseString(char[] s) {
        for(int i=0; i<s.length/2; i++){
            char tmp = s[s.length - 1 - i];
            s[s.length -1 - i] = s[i];
            s[i] = tmp;
        }
    }
}
