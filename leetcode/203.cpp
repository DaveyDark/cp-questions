/*
Question:
Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.
*/

// Approach:
// 1) We make a pointer curr and run it through the ll and also make a pointer to track the last element
// 2) If the current value is the target value then we skip the current node by modifying the previous node's next value
//    If prev is null then we move the head pointer to the next position to change the start of the list to the first non-target value
// 3) If the current element isn't the target value then we make it the previous element and step forward the current element
// 4) We return the head of the list at the end

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
    ListNode* removeElements(ListNode* head, int val) {
        ListNode *prev = nullptr,*curr = head;
        while(curr) {
            if(curr->val == val){
                if(prev) prev->next = curr->next;
                else head = curr->next;
            } else {
                prev = curr;
            }
            curr = curr->next;
        }
        return head;
    }
};
