/*
Question:
Given two linked list head1 and head2 with distinct elements, 
determine the count of all distinct pairs from both lists whose sum is equal to the given value x.

Note: A valid pair would be in the form (x, y) where x is from first linked list and y is from second linked list.
*/

// Approach:
// We create a frequency map of the first linked list. 
// Then we iterate through the second linked list and check if the frequency map contains the complement of the current element. 
// If it does, we add the frequency of the complement to the count.
// Finally, we return the count.

class Solution {

  public static int countPairs(LinkedList<Integer> head1, LinkedList<Integer> head2, int x) {
    HashMap<Integer, Integer> freq = new HashMap<>();
    
    for(int n: head1) {
      freq.put(n, freq.getOrDefault(n, 0) + 1);
    }
    
    int cnt = 0;
    
    for(int n: head2) {
      if(freq.containsKey(x - n)) {
        cnt += freq.get(x - n);
      }
    }
    
    return cnt;
  }
}
