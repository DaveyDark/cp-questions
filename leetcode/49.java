/*
Question:
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, 
typically using all the original letters exactly once.
*/

// Approach:
// 1) For each string, count the frequency of each character and store it in a string as space separated values.
// 2) Use a hashmap to store the strings with the same character frequency in a list.
// 3) Return the values of the hashmap as a list.

class Solution {
  String countChars(String s) {
    int[] freq = new int[26];
    for(char ch: s.toCharArray()) freq[ch - 'a']++;
    StringBuilder res = new StringBuilder();
    for(int f: freq) res.append(f + " ");
    return res.toString();
  }
  public List<List<String>> groupAnagrams(String[] strs) {
    HashMap<String, List<String>> res = new HashMap<>();
    for(String s: strs) {
      String key = countChars(s);
      List<String> l = res.getOrDefault(key, new ArrayList<String>());
      l.add(s);
      res.put(key, l);
    }
    return new ArrayList<>(res.values());
  }
}
