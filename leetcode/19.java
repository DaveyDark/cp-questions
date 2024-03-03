/*
Question:
Given the head of a linked list, remove the nth node from the end of the list and return its head.
*/

// Approach:
// We use two pointers, right and left
// The right pointer is n steps ahead of the left pointer
// Then, we move both pointers until right pointer is null
// And remove the node after left pointer

class Solution {
  public ListNode removeNthFromEnd(ListNode head, int n) {
    // Create two pointers, right and left
    ListNode right = head;
    ListNode left = head;

    // Move right pointer n steps ahead
    for(int i = 0;i < n; i++) {
      right = right.next;
    }

    // If right pointer is null, i.e. n == length of list
    // We need to remove the head, so return head.next
    if(right == null) return head.next;

    // Otherwise, move both pointer another step
    right = right.next;

    // Then, we move both pointers until right pointer is null
    while(right != null) {
      right = right.next;
      left = left.next;
    }

    // Finally, we remove the node after left pointer
    left.next = left.next.next;

    // And return the head
    return head;
  }
}
