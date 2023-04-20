/*
Question:
Given the root of a binary tree, invert the tree, and return its root.
*/

// Approach:
// We traverse each node in the tree and swap it's left and right children

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
    void invert(TreeNode *node){
        TreeNode *tmp = node->left;
        node->left = node->right;
        node->right = tmp;
    }
    void traverse(TreeNode *node){
        if(!node)return;
        invert(node);
        traverse(node->left);
        traverse(node->right);
    }
    TreeNode* invertTree(TreeNode* root) {
        traverse(root);
        return root;
    }
};
