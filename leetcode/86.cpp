/*
Question:
Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.

You should preserve the original relative order of the nodes in each of the two partitions.
*/

// Appraoch:
// 1) Make pointer for the head of the lesser part and greater part of the list and another pointer to track the last greater node
// 2) Traverse the list with a for loop
//    If the current node is less than x, 
//      Check and assign the lesser pointer and the head if they are unset
//      Otherwise set the next node of lesser to the current node and increment lesser
//      Then if the last pointer is valid then set it to teh next node to skip this node from the list
//    Otherwise check if the greater pointer is unassigned and set it to current if it is, and also update the last pointer
// 3) After the loop, we append the greater list to the end of the lesser list and return the head

class Solution {
public:
    ListNode* partition(ListNode* head, int x) {
        ListNode *lesser = nullptr, *last = nullptr, *greater = nullptr;
        for(ListNode *ptr = head; ptr; ptr = ptr->next) {
            if(ptr->val < x) {
                if(!lesser) {
                    lesser = ptr;
                    head = lesser;
                }
                else {
                    lesser->next = ptr;
                    lesser = lesser->next;
                }
                if(last) last->next = ptr->next;
            } else {
                if(!greater)greater = ptr;
                last = ptr;
            }
        }
        if(lesser)lesser->next = greater;
        return head;
    }
};
