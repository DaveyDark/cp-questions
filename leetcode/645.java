/*
Question:
You have a set of integers s,  which originally contains all the numbers from 1 to n. 
Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, 
which results in repetition of one number and loss of another number.

You are given an integer array nums representing the data status of this set after the error.

Find the number that occurs twice and the number that is missing and return them in the form of an array.
*/

// Approach:
// 1) Create a frequency array of size n
// 2) Iterate through the input array and increment the frequency of each element
// 3) Return the element that has a frequency of 2 and the element that has a frequency of 0

class Solution {
  public int[] findErrorNums(int[] nums) {
    int[] freq = new int[nums.length];
    int[] res = {0, 0};
    for(int n: nums) freq[n-1]++;
    for(int i = 0; i < freq.length; i++) {
      if(freq[i] == 2) res[0] = i+1;
      if(freq[i] == 0) res[1] = i+1;
    }
    return res;
  }
}
