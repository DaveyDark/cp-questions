/*
Question:
Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
*/

// Approach:
// 1) Create a hashmap to store the frequency of each element and populate it
// 2) Create a hashset to store the frequencies
// 3) Iterate through the hashmap and check if the frequency is already in the hashset
//    if it is, return false
//    else, add it to the hashset
// 4) If the loop finishes, return true

class Solution {
  public boolean uniqueOccurrences(int[] arr) {
    Map<Integer, Integer> map = new HashMap<>();
    for(int x: arr) {
      map.put(x, map.getOrDefault(x, 0) + 1);
    }
    Set<Integer> set = new HashSet<>();
    for(int f: map.values()) {
      if(set.contains(f)) return false;
      set.add(f);
    }
    return true;
  }
}
