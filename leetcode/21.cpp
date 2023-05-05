/*
You are given the heads of two sorted linked lists list1 and list2.
Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
Return the head of the merged linked list.
*/

// Approach
// 1) We make the head of the merged list and set it as an empty(dummy) node
// 2) We loop while both list1 and list2 are valid and append the smaller of the two to the head
//    And step the head and the smaller list
// 3) Outside the loop we append the remaining list
// 4) Then we return the merged list, but skip the first node because it is the dummy node

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
 // Iterative Approach
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode *head = new ListNode();
        ListNode *start = head;
        while (list1 && list2) {
            if (list1->val > list2->val){
                head->next = list2;
                list2 = list2->next;
            } else {
                head->next = list1;
                list1 = list1->next;
            }
            head = head->next;
        }
        if (list1) {
            head->next = list1;
        } else {
            head->next = list2;
        }
        return start->next;
    }
};

// Recursive Approach
class Solution {
public:
	ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) 
  {
		if(!l1)return l2;
		if(!l2)return l1;
		
		if(l1 -> val <= l2 -> val) {
			l1->next = mergeTwoLists(l1 -> next, l2);
			return l1;
		} else {
			l2->next = mergeTwoLists(l1, l2 -> next);
			return l2;            
		}
	}
};
