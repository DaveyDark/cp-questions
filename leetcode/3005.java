/*
Question:
You are given an array nums consisting of positive integers.

Return the total frequencies of elements in nums such that those elements all have the maximum frequency.

The frequency of an element is the number of occurrences of that element in the array.
*/

// Approach:
// We can just use a HashMap to count the frequency of each number, while keeping track of the maximum frequency
// And then return the result which is the product of the maximum frequency and the number of elements with maximum frequency

class Solution {
  public int maxFrequencyElements(int[] nums) {
    // Count frequency of each number, while keeping track of the maximum frequency
    HashMap<Integer, Integer> freq = new HashMap<>();
    int maxFreq = 0;
    for(int n: nums) {
      int f = freq.getOrDefault(n, 0) + 1;
      maxFreq = Math.max(f, maxFreq);
      freq.put(n, f);
    }

    // Count the number of elements with maximum frequency
    int cnt = 0;
    for(int v: freq.values()) {
      if(v == maxFreq) cnt++;
    }

    // Return the result which is the product of the maximum frequency and the number of elements with maximum frequency
    return cnt * maxFreq;
  }
}
