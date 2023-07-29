/*
Question:
In a linked list of size n, where n is even, the ith node (0-indexed) of the linked list is known as the twin of the (n-1-i)th node, if 0 <= i <= (n / 2) - 1.

    For example, if n = 4, then node 0 is the twin of node 3, and node 1 is the twin of node 2. These are the only nodes with twins for n = 4.

The twin sum is defined as the sum of a node and its twin.

Given the head of a linked list with even length, return the maximum twin sum of the linked list.
*/

// Approach:
// Use a stack to store the first half of the ll, 
// then after we reacht he mid point, we pop items from that stack one by one and calculate their sum by adding that to the value of the pointer
// 1) Make a stack
// 2) Go through the linked list two steps at a time, pushing the current value of head onto the stack and moving head one step
// 3) Until the stack is empty, 
//    Calculate the sum of the top of the stack with the value of head pointer
//    Set max to that value if it is greater.
//    Move head by one step and pop one element from the stack
// 4) Return the calculated max sum

#include<stack>
class Solution {
public:
    int pairSum(ListNode* head) {
        stack<int> stk;
        for(ListNode *ptr = head; ptr && ptr->next; ptr=ptr->next->next) {
            stk.push(head->val);
            head = head->next;
        }
        int max = 0;
        while(!stk.empty()) {
            max = std::max(stk.top() + head->val,max);
            head = head->next;
            stk.pop();
        }
        return max;
    }
};
