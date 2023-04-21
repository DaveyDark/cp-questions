/*
Question:
You are given the root of a binary search tree (BST) and an integer val.
Find the node in the BST that the node's value equals val and return the subtree rooted with that node. 
If such a node does not exist, return null.
*/

// Approach:
// 1) We recursively traverse the tree and return the root if it is null or the target node
// 2) Otherwise we search the left node if the value is lesser or the right node if it is greater

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
    TreeNode* searchBST(TreeNode* root, int val) {
        if(!root)return root;
        if(root->val == val) return root;
        else if (root->val > val)return searchBST(root->left,val);
        else return searchBST(root->right,val);
    }
};
