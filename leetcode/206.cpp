/*
Given the head of a singly linked list, reverse the list, and return the reversed list.
*/

// Approach:
// 1) We create a stack and push copies of all nodes in the list into it
// 2) We pop out nodes from the stack one by one until the stack is empty
//    The previous node is tracked and assigned the 'next' value of the current node
// 3) The next of the last node is set to null and the head is returned

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

#include <stack>

// Iterative Approach
class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        std::stack<ListNode*> stack;
        while(head){
            stack.push(new ListNode(head->val,head->next));
            head = head->next;
        }
        if(stack.empty())return nullptr;
        head = stack.top();
        ListNode *prevNode = stack.top(), *node;
        for(;!stack.empty();stack.pop()){
            node = stack.top();
            prevNode->next = node;
            prevNode = node;
        }
        node->next = nullptr;
        return head;
    }
};

// Recursive Approach
class Solution {
public:
    ListNode* reverseList(ListNode* head) {
        if (head == nullptr || head->next == nullptr) {
            return head;
        }
    
        ListNode* new_head = reverseList(head->next);
        head->next->next = head;
        head->next = nullptr;
        return new_head;
    }
};
