/*
Question:
Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
*/

// Approach
// 1) We make a 2d vector to store the level traversal
// 2) We traverse the tree keeping track of current depth and push the current node's value to the depth index in the vector
// 3) We return the vec at the end

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
    vector<vector<int>> vec;
    void traverse(TreeNode *node,int depth){
        if(!node)return;
        if(depth >= this->vec.size()) this->vec.resize(depth+1);
        this->vec.at(depth).push_back(node->val);
        traverse(node->left,depth+1);
        traverse(node->right,depth+1);
    }
    vector<vector<int>> levelOrder(TreeNode* root) {
        traverse(root,0);
        return this->vec;
    }
};
