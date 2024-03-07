/*
Question:
Given the head of a singly linked list, return the middle node of the linked list.

If there are two middle nodes, return the second middle node.
*/

// Approach:
// Make two pointers, slow and fast. 
// Slow moves one step at a time and fast moves two steps at a time. 
// When fast reaches the end, slow will be at the middle of the list.
// So can return slow.

class Solution {
  public ListNode middleNode(ListNode head) {
    // Create the two pointers
    ListNode slow = head, fast = head;

    // Loop until fast reaches the end of the list
    while(fast != null && fast.next != null) {
      // Move slow one step and fast two steps
      fast = fast.next.next;
      slow = slow.next;
    }

    // Return the slow pointer
    return slow;
  }
}
