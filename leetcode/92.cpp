/*
Question:
Given the head of a singly linked list and two integers left and right where left <= right, 
reverse the nodes of the list from position left to position right, and return the reversed list.
*/

// Approach:
// 1) Make pointers to track the start and end of region to be reversed.
// 2) Go through the list and set start and end
// 3) Make a pointer to the remainder of the list
// 4) If start exists i.e. we have to slice from the middle, reverse the given region,
//     Set the ending node's next pointer to tail
//     Then set end as the start
//   If it doesn't i.e. we have to slice from the start, reverse the given region
//    Set the next of the head to tail
//    Then set the head to end
// 5) Return the head

class Solution {
public:
    void reverse(ListNode *node, ListNode *end) {
        if(node == end || !node->next) return;
        reverse(node->next, end);
        node->next->next = node;
        node->next = nullptr;
    }
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode *start = nullptr, *end = nullptr;
        int i = 1;
        for(ListNode *p = head; p && !end; p = p->next, i++) {
            if(i==left-1) start = p;
            if(i==right) end = p;
        }
        ListNode *tail = end->next;
        if(start) {
            reverse(start->next,end);
            start->next->next = tail;
            start->next = end;
        }
        else {
            reverse(head,end);
            head->next = tail;
            head = end;
        }
        return head;
    }
};
