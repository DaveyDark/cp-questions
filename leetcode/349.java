/*
Question:
Given two integer arrays nums1 and nums2, return an array of their intersection. 
Each element in the result must be unique and you may return the result in any order.
*/

// Approach:
// We can convert both arrays to sets and use the retainAll method to find the intersection of the two sets.
// The intersection will be the set of elements that are present in both arrays.
// Then we can convert the set to an array and return it.

class Solution {
  public int[] intersection(int[] nums1, int[] nums2) {
    // Init the two sets
    Set<Integer> s1 = new HashSet<>();
    Set<Integer> s2 = new HashSet<>();

    // Populate the two sets with elements from the arrays
    for(int n: nums1)
      s1.add(n);
    for(int n: nums2)
      s2.add(n);

    // Calculate intersection
    s1.retainAll(s2);

    // Convert the set to an array
    int[] res = new int[s1.size()];
    int i = 0;
    for(int n: s1)
      res[i++] = n;

    // Return the array
    return res;
  }
}
