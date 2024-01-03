/*
Question:
Anti-theft security devices are activated inside a bank. 
You are given a 0-indexed binary string array bank representing the floor plan of the bank, 
which is an m x n 2D matrix. 
bank[i] represents the ith row, consisting of '0's and '1's. 
'0' means the cell is empty, while'1' means the cell has a security device.

There is one laser beam between any two security devices if both conditions are met:
    The two devices are located on two different rows: r1 and r2, where r1 < r2.
    For each row i where r1 < i < r2, there are no security devices in the ith row.

Laser beams are independent, i.e., one beam does not interfere nor join with another.

Return the total number of laser beams in the bank.
*/

// Approach:
// 1) Init last and beams to 0
// 2) Iterate through the bank and count the number of ones in each string
// 3) If the count is greater than 0, then add the product of the last count and the current count to the total number of beams

class Solution {
  int countOnes(String s) {
    int sum = 0;
    for (int i = 0; i < s.length(); i++) {
      sum += (s.charAt(i) == '1') ? 1 : 0;
    }
    return sum;
  }

  public int numberOfBeams(String[] bank) {
    int last = 0;
    int beams = 0;
    for (String s : bank) {
      int count = countOnes(s);
      if (count > 0) {
        beams += last * count;
        last = count;
      }
    }
    return beams;
  }
}
