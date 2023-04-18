/*
Question:
Given the root of a binary tree, return the postorder traversal of its nodes' values.
*/

// Approach:
// 1) We do preorder traversal using recursion in the traverse function
// 2) If the current node is null we return
// 3) Otherwise we traverse the left node if it exists, then traverse the right node if it exists and then push the current node's value and


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
    vector<int> vec;
    void traverse(TreeNode *node){
        if(!node)return;
        if(node->left)traverse(node->left);
        if(node->right)traverse(node->right);
        this->vec.push_back(node->val);
    }
    vector<int> postorderTraversal(TreeNode* root) {
        traverse(root);
        return this->vec;
    }
};
