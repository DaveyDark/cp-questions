/*
Question:
You are given a 0-indexed array nums consisting of positive integers.

There are two types of operations that you can apply on the array any number of times:
    Choose two elements with equal values and delete them from the array.
    Choose three elements with equal values and delete them from the array.

Return the minimum number of operations required to make the array empty, or -1 if it is not possible.
*/

// Approach:
// 1) Make a HashMap and count the frequency of each element using the map.
// 2) Iterate through the frequencies and count the number of operations required.
//    If the frequency is 1, return -1.
//    If the frequency is divisible by 3, add (freq/3) to the answer.
//    Otherwise, add (freq/3) + 1 to the answer.
// 3) Return the answer.

import java.util.*;

class Solution {
  public int minOperations(int[] nums) {
    Map<Integer, Integer> freq = new HashMap<Integer, Integer>();

    for(int n: nums) {
      if(freq.containsKey(n)) {
        freq.put(n, freq.get(n)+1);
      } else {
        freq.put(n, 1);
      }
    }

    int ops = 0;
    for(int v: freq.values()) {
      if(v == 1) return -1;
      if(v%3 == 0) {
        ops += (v/3);
      } else ops += (v/3) + 1;
    }
    
    return ops;
  }
}
