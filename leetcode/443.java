/*
Question:
Given an array of characters chars, compress it using the following algorithm:
Begin with an empty string s. For each group of consecutive repeating characters in chars:
    If the group's length is 1, append the character to s.
    Otherwise, append the character followed by the group's length.
The compressed string s should not be returned separately, but instead, be stored in the input character array chars. 
Note that group lengths that are 10 or longer will be split into multiple characters in chars.
After you are done modifying the input array, return the new length of the array.
You must write an algorithm that uses only constant extra space.
*/

// Approach:
// 1) We make variables to keep track of the current streak of consecutive characters and the insertion index
// 2) We go over the array, Each iteration we check if it is the last char of the string or if the current char is different from the last char
//    If either of this is true, we end the streak by adding the char at the cursor and incrementing it
//    If we have an actual streak of > 1, then we append the digits of the streak at the cursor and move it further forward
//    Otherwise we increment the streak
// 3) At the end we return the last position of the cursor as the length of the new array

class Solution {
    public int compress(char[] chars) {
        int streak = 1,cursor = 0;
        for(int i=1; i<=chars.length; i++){
            if(i==chars.length || chars[i-1] != chars[i]) {
                chars[cursor++] = chars[i-1];
                if(streak>1){
                    String digits = String.valueOf(streak);
                    for(int j=0; j<digits.length(); j++)
                        chars[cursor++] = digits.charAt(j);
                    streak = 1;
                }
            } else streak++;
        }
        return cursor;
    }
}
