/*
Question:
A linked list of length n is given such that each node contains an additional random pointer, which could point to any node in the list, or null.

Construct a deep copy of the list. The deep copy should consist of exactly n brand new nodes, where each new node has its value set to the value of its corresponding original node. Both the next and random pointer of the new nodes should point to new nodes in the copied list such that the pointers in the original list and copied list represent the same list state. None of the pointers in the new list should point to nodes in the original list.

For example, if there are two nodes X and Y in the original list, where X.random --> Y, then for the corresponding two nodes x and y in the copied list, x.random --> y.

Return the head of the copied linked list.

The linked list is represented in the input/output as a list of n nodes. Each node is represented as a pair of [val, random_index] where:
  val: an integer representing Node.val
  random_index: the index of the node (range from 0 to n-1) that the random pointer points to, or null if it does not point to any node.

Your code will only be given the head of the original linked list.
*/

// Approach:
// 1) If the head is null we return it
// 2) Create the head of the new list and set it's value to the original head's value.
//    Also create a hashmap. The hashmap will store the equivalent copy of a node in the original list
// 3) Go through the original list and copy it by adding new nodes in the new list and setting their values from the original
//    Also populate the hashmap alongside
// 4) Go through the list again and using the hashmap,
//    set the random pointer of nodes in the new list to the equivalent copy of the node the random pointer of the original list points to
// 5) Return the head of the new linked list

#include <unordered_map>

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if(!head) return head;
        Node *new_head = new Node(head->val);
        unordered_map<Node*, Node*> map;
        for(Node *pnt = head, *pnt2 = new_head; pnt; pnt = pnt->next, pnt2 = pnt2->next) {
            pnt2->val = pnt->val;
            map[pnt] = pnt2;
            if(pnt->next)pnt2->next = new Node(0);
        }
        for(Node *p = head, *p2 = new_head; p; p = p->next, p2 = p2->next) {
            p2->random = map[p->random];
        }
        return new_head;
    }
};
