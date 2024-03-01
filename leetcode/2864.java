/*
Question:
You are given a binary string s that contains at least one '1'.

You have to rearrange the bits in such a way that the resulting binary number is the maximum odd binary number that can be created from this combination.

Return a string representing the maximum odd binary number that can be created from the given combination.

Note that the resulting string can have leading zeros.
*/

// Approach:
// We can count the number of ones in the input string, 
// then add the ones to the result string until we have only one left. 
// The last one will be put at the end of the string. 

class Solution {
  public String maximumOddBinaryNumber(String s) {
    // String Builder to build the result string
    StringBuilder sb = new StringBuilder();

    // Count the number of ones in the input string
    int ones = 0;
    for(char ch: s.toCharArray()) {
      if(ch == '1') ones++;
    }

    for(int i = 0; i < s.length()-1; i++) {
      if(ones > 1) {
        // If we have spare ones, we add them to the string
        ones--;
        sb.append('1');
      } else {
        // Otherwise we add a 0
        sb.append('0');
      }
    }
    // The last 1 will be put at the end
    sb.append('1');

    // Return the result string from the StringBuilder
    return sb.toString();
  }
}
