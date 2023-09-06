/*
Question:
Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.

The length of each part should be as equal as possible: no two parts should have a size differing by more than one. 
This may lead to some parts being null.

The parts should be in the order of occurrence in the input list, 
and parts occurring earlier should always have a size greater than or equal to parts occurring later.

Return an array of the k parts.
*/

// Approach:
// 1) Make a vector to store the splits and go over the whole list once and calculate the length
// 2) Divide the length by k to find the split size and remainder
// 3) Push the head into the splits vector. Init a pointer for tail and set it to nullptr
// 4) Go over the linked list again, keeping track of current partition length in cnt
//     Each iteration, if the tail of a partition has been found, we cut it off,
//     then push the current node to the splits vector as a new head for the current partition and reset cnt and tail
//    After that, check if the current partition size is more than the split size
//     Check if remainder is positive, if it is then we can add another element to the partition so we just decrease remainder
//     If it isn't then set the tail to the current node since the partition should end here
// 5) If the number of splits doesn't match k then we push nullptr into splits until it does
// 6) Return the splits vector

class Solution {
public:
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        vector<ListNode*> splits;
        int len = 0;
        for(ListNode *p = head; p; p = p->next, len++);
        ListNode *tail = nullptr;
        int split_size = len/k, remainder = len%k, cnt = 1;
        splits.push_back(head);
        for(ListNode *p = head; p; p = p->next, cnt++) {
            if(tail) {
                tail->next = nullptr;
                tail = nullptr;
                splits.push_back(p);
                cnt = 1;
            }
            if(cnt >= split_size) {
                if(remainder > 0 && cnt == split_size) remainder--;
                else tail = p;
            }
        }
        while(splits.size() != k)splits.push_back(nullptr);
        return splits;
    }
};
