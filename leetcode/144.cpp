/*
Given the root of a binary tree, return the preorder traversal of its nodes' values.
*/

// Approach:
// 1) We use reecursion to traverse the tree using the traverse function
// 2) We return if the node is null, otherwise we push the current value into the vector
// 3) Then we check if the node has any children and it it has then we call traverse on those

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
        this->vec.push_back(node->val);
        if(node->left)traverse(node->left);
        if(node->right)traverse(node->right);
    }
    vector<int> preorderTraversal(TreeNode* root) {
        traverse(root);
        return this->vec;
    }
};
