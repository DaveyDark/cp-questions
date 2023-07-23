/*
Question:
You are given the head of a linked list. Delete the middle node, and return the head of the modified linked list.

The middle node of a linked list of size n is the ⌊n / 2⌋th node from the start using 0-based indexing, 
where ⌊x⌋ denotes the largest integer less than or equal to x.

    For n = 1, 2, 3, 4, and 5, the middle nodes are 0, 1, 1, 2, and 2, respectively.
*/

// Approach:
// 1) Make two pointers, one to traverse the ll and the other to store the mid
// 2) Loop through the ll, keeping a counter i
//    Increment ptr each iteration
//    Increment mid every other iteration but only if ptr-> and thus ptr is not null
//    This ensures that mid points to one element before the exact middle of the list
// 3) If mid or the next node of mid is null, return a nullptr because the new list will be empty
// 4) If the lsit has only two elements then just detach the last element
// 5) If neither of the above is true, which means the list has more than two elements,
//    We make mid's next point to the next next node, skipping and hence deleting the middle node
// 6) Return the head of the linked list at the end, the mid element has been removed

class Solution {
public:
    ListNode* deleteMiddle(ListNode* head) {
        ListNode *mid = head, *ptr = head;
        for (int i=0; ptr != nullptr; i++)
        {
            ptr = ptr->next;
            if(i%2!=0 && ptr && ptr->next)mid = mid->next;
        }
        if(!mid || !mid->next) return nullptr;
        else if(mid->next && !mid->next->next) mid->next = nullptr;
        else mid->next = mid->next->next;
        return head;
    }
};
