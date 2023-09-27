/*
Question:
You are given an encoded string s. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:
    If the character read is a letter, that letter is written onto the tape.
    If the character read is a digit d, the entire current tape is repeatedly written d - 1 more times in total.

Given an integer k, return the kth letter (1-indexed) in the decoded string.
*/

// Approach:
// 1) Calculate the length of the decoded string by iterating over the input string s.
//    - If a digit is encountered, update the decodedLength by multiplying it by the digit.
//    - If a non-digit character is encountered, increment the decodedLength by 1.
// 2) Iterate over the input string s in reverse order to find the k-th character:
//    - If a digit is encountered, adjust the decodedLength and k accordingly based on the digit.
//    - If a non-digit character is encountered:
//      - If k is 0 or equal to the remaining decodedLength, return the current character.
//      - Otherwise, decrement the decodedLength by 1.
// 3) If k is out of bounds (less than 0 or greater than the length of the decoded string), return an empty string.

class Solution {
  public String decodeAtIndex(String s, int k) {
    long decodedLength = 0;
    int n = s.length();

    for (int i = 0; i < n; i++) {
      char ch = s.charAt(i);
      if (Character.isDigit(ch)) {
        int repeat = ch - '0';
        decodedLength *= repeat;
      } else {
        decodedLength++;
      }
    }

    for (int i = n - 1; i >= 0; i--) {
      char ch = s.charAt(i);
      if (Character.isDigit(ch)) {
        int repeat = ch - '0';
        decodedLength /= repeat;
        k %= decodedLength;
      } else {
        if (k == 0 || k == decodedLength) {
          return String.valueOf(ch);
        }
        decodedLength--;
      }
    }

    return "";
  }
}
