/*
Question:
Given the head of a linked list, we repeatedly delete consecutive sequences of nodes that sum to 0 until there are no such sequences.

After doing so, return the head of the final linked list.  You may return any such answer.

(Note that in the examples below, all sequences are serializations of ListNode objects.)
*/

// Approach:
// We can maintain a prefix sum while traversing the list and if a value is repeated, we have a zero sum sublist.
// When we encounter a repeated sum, we can remove the nodes from the node that has the same sum to the current node.

class Solution {
  public ListNode removeZeroSumSublists(ListNode head) {
    // HashMap to store the sum and the node that has that sum
    HashMap<Integer, ListNode> map = new HashMap<>();
    // Insert 0 sum and null node to handle the case when the sum becomes 0 i.e. when the head is to be removed
    map.put(0, null);
    // Prefix sum
    int sum = 0;

    // Iterate through the list
    for(ListNode node = head; node != null; node = node.next) {
      // Add the current node's value to the sum
      sum += node.val;

      // If the sum is repeated, we have a zero sum sublist
      if(map.containsKey(sum)) {
        // The start of the sublist is the next node of the node that has the same sum
        ListNode n = map.get(sum);
        
        if(n == null) {
          // If the start of the sublist is null, remove the nodes from the head to the current node
          removeNodesInRange(head, node, map, sum);
          head = node.next;
        } else {
          // Otherwise, remove the nodes from the next node of the start to the next node of the current node
          removeNodesInRange(n.next, node, map, sum);
          n.next = node.next;
        }
      } 
      else 
        // Otherwise, add the sum and the node to the map
        map.put(sum, node);
    }

    return head;
  }

  private void removeNodesInRange(ListNode start, ListNode end, HashMap<Integer, ListNode> map, int sum) {
    // Traverse the list from start to end, tracking the prefix sum and removing the sum from the map
    for(ListNode curr = start; curr != end; curr = curr.next) {
      sum += curr.val;
      map.remove(sum);
    }
  }
}
