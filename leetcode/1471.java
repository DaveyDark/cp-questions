/*
Question:
Given an array of integers arr and an integer k. 
Find the least number of unique integers after removing exactly k elements.
*/

// Approach:
// 1) Create a HashMap to store the frequency of each element in the array
// 2) Create a list from the values of the HashMap
// 3) Sort the list
// 4) Iterate through the list and subtract the value of current element from k
//    If k becomes negative, break the loop
//    Otherwise, increment the counter
// 5) Return the difference of the size of the list and the counter

class Solution {
  public int findLeastNumOfUniqueInts(int[] arr, int k) {
    Map<Integer, Integer> freq = new HashMap<>();
    for(int x: arr) freq.put(x, freq.getOrDefault(x, 0) + 1);
    List<Integer> f = new ArrayList<>(freq.values());
    Collections.sort(f);
    int i = 0;
    for(int x: f) {
      k -= x;
      if(k < 0) break;
      i += 1;
    }
    return f.size() - i;
  }
}
