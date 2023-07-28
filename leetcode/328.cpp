/*
Question:
Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, 
and return the reordered list.
The first node is considered odd, and the second node is even, and so on.
Note that the relative order inside both the even and odd groups should remain as it was in the input.
You must solve the problem in O(1) extra space complexity and O(n) time complexity.
 */

// Approach:
// 1) Return head if the list is empty or has a single node
// 2) Make variables to store the odd and even area traversal pointers and head of the even section 
// 3) Go over the linked list with a counter i
//     If i is even
//      If even_point does not exist the set it and even_head to ptr
//      If even_point does exist then make the current node it's next node and move it forward
//     Similar procedure if i is odd for odd_point
// 4) Connect odd list to even list by making even_head the next node of odd_point and 
//    Set the next node of even_point to be null to terminate the list
// 5) Return back the head

class Solution {
public:
    ListNode* oddEvenList(ListNode* head) {
        if(!head || !head->next) return head;
        ListNode *odd_point = nullptr, *even_point = nullptr, *even_head = nullptr;
        int i = 1;
        for(ListNode *ptr = head; ptr; ptr = ptr->next,i++) {
            if(i%2 == 0) {
                //even
                if(!even_point) {
                    even_point = ptr;
                    even_head = ptr;
                    continue;
                }
                even_point->next = ptr;
                even_point = ptr;
            } else {
                //odd
                if(!odd_point) {
                    odd_point = ptr;
                    continue;
                }
                odd_point->next = ptr;
                odd_point = ptr;
            }
        }
        odd_point->next = even_head;
        even_point->next = nullptr;
        return head;
    }
};
