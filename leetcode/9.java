/*
Question:
Given an integer x, return true if x is a palindrome, and false otherwise.
Follow up: Could you solve it without converting the integer to a string?
*/

// Approach:
// 1) We make a function to reverse the given int
// 2) We check if the reverse of x is equal to x and return true if it is and false other wise

class Solution {
    public int reverse(int x) {
        int rev = 0;
        for(; x>0; x/=10) rev = (rev*10) + (x%10);
        return rev;
    }
    public boolean isPalindrome(int x) {
        return this.reverse(x) == x;
    }
}
