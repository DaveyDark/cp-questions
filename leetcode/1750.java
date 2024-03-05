/*
Question:
Given a string s consisting only of characters 'a', 'b', and 'c'. You are asked to apply the following algorithm on the string any number of times:
    Pick a non-empty prefix from the string s where all the characters in the prefix are equal.
    Pick a non-empty suffix from the string s where all the characters in this suffix are equal.
    The prefix and the suffix should not intersect at any index.
    The characters from the prefix and suffix must be the same.
    Delete both the prefix and the suffix.

Return the minimum length of s after performing the above operation any number of times (possibly zero times).
*/

// Approach:
// Use two pointers, left and right, to traverse the string from both ends.
// Compare the characters at the left and right pointers.
// If they are not equal, return the current distance between left and right pointers
// Otherwise, move both pointers until they are different or go out of range.
// If the pointers have crossed each other, return 0, otherwise return 1.

class Solution {
  public int minimumLength(String s) {
    int l = s.length();

    // Initialize left and right pointers
    int left = 0;
    int right = l - 1;

    // Loop until left and right pointers meet
    while (left < right) {
      // Store current character
      char ch = s.charAt(left);

      // If the left and right chars are not equal, return the length of the substring
      if (ch != s.charAt(right)) {
        return right - left + 1;
      }

      // Otherwise, move both pointers until they are different or go out of range
      while (left < l && s.charAt(left) == ch)
        left++;
      while (right > 0 && s.charAt(right) == ch)
        right--;
    }

    // If the pointers have crossed each other, return 0, otherwise return 1
    return (left > right) ? 0 : 1;
  }
}
