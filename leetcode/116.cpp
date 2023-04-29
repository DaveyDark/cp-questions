/*
Question:
You are given a perfect binary tree where all leaves are on the same level, and every parent has two children. The binary tree has the following definition:
struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.
Initially, all next pointers are set to NULL.
*/

// Approach:
// 1) We make a vector to store all the nodes in the current level and a queue to traverse the tree breadth first
// 2) We loop until the queue is empty
// 3) We pop the first node from the queue and check if it is nullptr
// 4) If it is then we have reached the end of level and we assign the next attributes by going through the level vector and then we empty it
//    Then we push another nullptr to the end if the queue isn't empty
// 5) Else we push the current node into the level and then add the left and right children into the queue

/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* left;
    Node* right;
    Node* next;

    Node() : val(0), left(NULL), right(NULL), next(NULL) {}

    Node(int _val) : val(_val), left(NULL), right(NULL), next(NULL) {}

    Node(int _val, Node* _left, Node* _right, Node* _next)
        : val(_val), left(_left), right(_right), next(_next) {}
};
*/

class Solution {
public:
    Node* connect(Node* root) {
        if (!root) return nullptr;
        
        vector<Node*> level;
        queue<Node*> q;
        
        q.push(root);
        q.push(nullptr);

        while (!q.empty()) {
            Node *front = q.front();
            q.pop();
            if (front == nullptr) {
                int n = level.size();
                for (int i=0; i<n-1; i++) level[i]->next = level[i+1];
                level.clear();
                if (!q.empty()) q.push(nullptr);
            } 
            else{
                level.push_back(front);
                if (front->left) q.push(front->left);
                if (front->right) q.push(front->right);
            }

        }
        return root;
    }
};
