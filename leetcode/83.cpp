/*
Question:
Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
*/

// Approach:
// 1) We store the starting node of the linked list and then loop through until the head exists
// 2) Each iteration we check if the next node exists and if it does we check if it's value is the same as the current node
//    If it is then we set the next node to the next next node, basically skipping the next node from the list
// 3) Otherwise we step the head
// 4) We return the original start of the linked list at the end

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* deleteDuplicates(ListNode* head) {
        ListNode *start = head;
        while (head) {
            if(head->next && head->next->val == head->val)head->next = head->next->next;
            else head = head->next;
        }
        return start;
    }
};
