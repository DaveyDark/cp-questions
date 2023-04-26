/*
Question:
Given the head of a singly linked list, return the middle node of the linked list.
If there are two middle nodes, return the second middle node.
*/

// Approach:
// If two pointers are made and one is made to travel at double speed, 
// then by the time the fast pointer reaches the end of the LL, the slow one would be at the middle
// 1) We make a fast and slow pointer and set it to the head of the LL
// 2) We advance fast by 2 and slow by 1 steps while fast and fast->next are valid
// 3) We return slow at the end

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
    ListNode* middleNode(ListNode* head) {
        ListNode *slow = head, *fast = head;
        while(fast && fast->next){
            fast = fast->next->next;
            slow = slow->next;
        }
        return slow;
    }
};
