/*
Question:
Given two binary strings a and b, return their sum as a binary string.
*/

// Approach:
// 1) We make a string builder to build the resulting string, along with len to store the bigger of the lengths of the two strings
//    and also sum and carry
// 2) Then we loop until i is less than len and carry is non zero, and each iteration we set sum to the last value of carry,
//    Then add the digits from both of the strings if they are winthin the length of the string
//    We add digits from the last because the digits of a number are added from right to left
//    Then we append the remainder of sum to ans and set carry to sum/2
// 3) At the end we reverse ans and return it as a string

class Solution {
    public String addBinary(String a, String b) {
        StringBuilder ans = new StringBuilder();
        int len = Math.max(a.length(), b.length());
        int sum=0, carry=0;
        for(int i=0; i<len || carry!=0; i++) {
            sum = carry;
            if(i<a.length()) sum += a.charAt(a.length() -i -1)-'0';
            if(i<b.length()) sum += b.charAt(b.length() -i -1)-'0';
            ans.append(sum%2);
            carry = sum/2;
        }
        return ans.reverse().toString();
    }
}
