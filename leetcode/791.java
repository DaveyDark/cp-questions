/*
Question:
You are given two strings order and s. All the characters of order are unique and were sorted in some custom order previously.

Permute the characters of s so that they match the order that order was sorted. 
More specifically, if a character x occurs before a character y in order, then x should occur before y in the permuted string.

Return any permutation of s that satisfies this property.
*/

// Approach:
// We count the frequency of each character in s and store it in a HashMap
// And then we iterate through the order string and append the characters to the result string with their frequency in s
// Finally we append the remaining characters in s to the result string

class Solution {
  public String customSortString(String order, String s) {
    // HashMap to store frequency of each character in s
    HashMap<Character, Integer> freq = new HashMap<>();

    // Populate the freq map
    for(char ch: s.toCharArray()) {
      freq.put(ch, freq.getOrDefault(ch, 0) + 1);
    }

    // Create a StringBuilder to store the result
    StringBuilder res = new StringBuilder(s.length());

    // Iterate through the order string and append the characters to res with their frequency in s
    for(char ch: order.toCharArray()) {
      int f = freq.getOrDefault(ch, 0);
      while(f-- > 0) res.append(ch);
      freq.put(ch, 0);
    }

    // Append the remaining characters in s to res
    for(Map.Entry<Character, Integer> e: freq.entrySet()) {
      int v = e.getValue();
      while(v-- > 0) res.append(e.getKey());
    }

    // Return the result
    return res.toString();
  }
}
