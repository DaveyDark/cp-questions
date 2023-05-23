/*
Question:
You are given two non-empty linked lists representing two non-negative integers. 
The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.
*/

// Approach:
// We add each node and use the singles place digit as the value and the tens place digit as carry,
// which is then added to the value of the next node
// 1) We loop while either l1 or l2 exists
// 2) We calculate sum by adding carry to the values of l1 and l2 of they exist
// 3) Then we calculate the new carry and set the value of current node in sol to the singles place digit of sum
// 4) Then we advance both lists if they exist, and if the loop isn't ending, i.e. one of l1 or l2 exists, we advance the sol ll
// 5) Outside the loop, if there' still a carry then we append it to the end of sol, and then return the head of the solution ll

class Solution {
public:
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        int carry=0;
        ListNode *solHead = new ListNode();
        ListNode *sol = solHead;
        while(l1 || l2) {
            int sum = carry + (l1 ? l1->val : 0) + (l2 ? l2->val : 0);
            carry = sum/10;
            sol->val = sum%10;
            
            if(l1)l1 = l1->next;
            if(l2)l2 = l2->next;
            
            if(l1 || l2){
                sol->next = new ListNode();
                sol = sol->next;
            }
        }
        if(carry != 0)sol->next = new ListNode(carry);
        return solHead;
    }
};
