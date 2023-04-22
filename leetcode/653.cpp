/*
Question:
Given the root of a binary search tree and an integer k, 
return true if there exist two elements in the BST such that their sum is equal to k, or false otherwise.
*/

// Approach:
// 1) We traverse the tree inorder to get a sorted vector of the elements
// 2) Then we take two pointers, one at the start and other at the end, and move them closer to each other
//    Until either their sum is target or they become equal
// 3) We return false at the end of the function

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    void traverse(TreeNode *node,vector<int> *vec){
        if(!node)return;
        traverse(node->left,vec);
        vec->push_back(node->val);
        traverse(node->right,vec);
    }
    bool findTarget(TreeNode* root, int k) {
        if(!root)return false;
        vector<int> list;
        traverse(root,&list);

        int *start = &list[0],*end = &list[list.size()-1];
        while(start != end) {
            if(*start + *end == k)return true;
            else if (*start + *end > k){
                end--;
            } else {
                start++;
            }
        }
        return false;
        
    }
};
