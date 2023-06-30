/*
Question:
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
The algorithm for myAtoi(string s) is as follows:
    Read in and ignore any leading whitespace.
    Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
    Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
    Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
    If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
    Return the integer as the final result.
Note:
    Only the space character ' ' is considered a whitespace character.
    Do not ignore any characters other than the leading whitespace or the rest of the string after the digits
*/

// Approach:
// We have just implemented the given algorithm
// 1) We trim the string to remove leading whitespaces, and return 0 is the string is empty after trimming
// 2) We check if the sign is specified in the string, and again return 0 is the string contains only the sign
// 3) Then if there is a sign included in the string, we set start to 1, which is the index we start reading digits from
// 4) Then we go over the rest of the string, and each iteration, we break out of the loop if the current char isn't a digit
//    Otherwise we find out the digit we want to add, then check if it is greater than the max int value / 10
//    or if it is equal to max value / 10 but greater than 7
//    In there cases, it will cause an overflow so we return the max or min value depending on the sign
//    Otherwise we just multiply the num by 10 and add the digit
// 5) Outside the loop, we return the num, with a negative sign if the string was negative and positive otherwise

class Solution {
    public int myAtoi(String s) {
        s = s.trim();
        if(s.length() == 0)return 0;
        boolean negative = (s.charAt(0) == '-');
        int start = 0;
        int num = 0;
        if(s.charAt(0) == '-' || s.charAt(0) == '+')start = 1;
        if(s.length() - start == 0) return 0;
        for(int i = start; i < s.length(); i++) {
            if(!Character.isDigit(s.charAt(i))){
                break;
            } else {
                int digit = s.charAt(i) - '0';
                if (num > Integer.MAX_VALUE / 10 || (num == Integer.MAX_VALUE / 10 && digit > 7)) {
                    return negative ? Integer.MIN_VALUE : Integer.MAX_VALUE;
                }
                num = (num*10) + digit;
            }
        }
        if(negative) return -num;
        return num;
    }
}
