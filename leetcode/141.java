/*
Question:
Given head, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. 
Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.

Return true if there is a cycle in the linked list. Otherwise, return false.
*/

// Approach:
// We can use two pointers, slow and fast. 
// The slow pointer moves by 1 and the fast pointer moves by 2. 
// If there is a cycle, the fast pointer will meet the slow pointer at some point.
// Otherwise, the fast pointer will reach the end of the list and we can return false.

public class Solution {
  public boolean hasCycle(ListNode head) {
    // Make a slow and fast pointer, initialize to head
    ListNode slow = head, fast = head;

    // Loop until fast is null or fast.next is null
    while(fast != null && fast.next != null) {
      // Move fast by 2 and slow by 1
      fast = fast.next.next;
      slow = slow.next;

      // If fast and slow meet, there is a cycle, so we return true
      if(fast == slow) return true;
    }

    // If we reached the end of the list, there is no cycle, so we return false
    return false;
  }
}
