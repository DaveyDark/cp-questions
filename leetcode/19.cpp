/*
Question:
Given the head of a linked list, remove the nth node from the end of the list and return its head.
*/

// Approach:
// 1) We make a temp pointer that points to the head to simulate deletion
// 2) We make two pointers, lead and follow and set them to tmp
// 3) We move the lead pointer ahead by n steps
// 4) Then we move both pointers forward until lead reaches end of array
// 5) Now, follow should be n steps from the end, so we just skip the next element of follow
// 6) We return tmp->next. head is not returned because head of the list after deletion could be changed;

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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode *tmp = new ListNode();
        tmp->next = head;
        ListNode *lead = tmp, *follow = tmp;
        for(n;n>0;n--)lead = lead->next;
        while(lead && lead->next){
            lead = lead->next;
            follow = follow->next;
        }
        follow->next = follow->next->next;
        return tmp->next;
    }
};
