/*
Question:
Given head, the head of a linked list, determine if the linked list has a cycle in it.
There is a cycle in a linked list if there is some node in the list that can be reached again 
by continuously following the next pointer. Internally, pos is used to denote the index of the node 
that tail's next pointer is connected to. Note that pos is not passed as a parameter.
Return true if there is a cycle in the linked list. Otherwise, return false.
*/

// Approach:
// If we use two pointers, one moving 1 step at a time and the other moving 2 steps at a time,
// They will eventually be equal if there is a cycle
// 1) We take two pointers, slow and fast and set them to head
// 2) While fast and fast->next exist we loop
// 3) inside the loop we move fast by 2 steps and slow by 1 step
// 4) If at any iteration slow == fast, then there is a cycle and we return true
// 5) Otherwise if we get outside the loop we return false;

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */

class Solution {
public:
    bool hasCycle(ListNode *head) {
        ListNode *slow = head;
        ListNode *fast = head;
        while (fast && fast->next) {
            fast = fast->next->next;
            slow = slow->next;
            if (fast == slow) {
                return true;
            }
        }
        return false;
    }
};
