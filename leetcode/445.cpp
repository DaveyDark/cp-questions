/*
Question:
You are given two non-empty linked lists representing two non-negative integers. 
The most significant digit comes first and each of their nodes contains a single digit. 
Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

// Approach:
// 1) We first reverse both given lists
// 2) Then we go through both the lists simulataneously, 
//    addiing the values from both the lists if the exist, 
//    then calculating sum and carry based on that
//    then we push the sum to l3
//    and end by incrementing l1,l2 and l3 if they are valid
// 3) After the loop we reverse l3 and return it

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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        l1 = reverse(l1);
        l2 = reverse(l2);
        ListNode* l3_head = new ListNode();
        ListNode* l3 = l3_head;
        int carry = 0;
        while(l1 || l2 || carry) {
            int n1 = l1? l1->val:0;
            int n2 = l2? l2->val:0;
            int sum = (n1+n2+carry)%10;
            carry = (n1+n2+carry)/10;
            l3->val = sum;

            if(l1) l1 = l1->next;
            if(l2) l2 = l2->next;
            if(l1||l2||carry) l3->next = new ListNode();

            l3 = l3->next;
        }
        return reverse(l3_head);
    }
private:
    ListNode* reverse(ListNode* node) {
        if(!node || !node->next) return node;
        ListNode *newHead = reverse(node->next);
        node->next->next = node;
        node->next = nullptr;
        return newHead;
    }
};
