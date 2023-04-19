/*
Question:
Given the root of a binary tree, return its maximum depth.
A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
*/

// Approach:
// We traverse the tree keeping note of the current depth and storing the max value of it and return the max depth at the end

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
    int max;
    void traverse(TreeNode* node,int depth){
        if(!node)return;
        if(depth>this->max)this->max = depth;
        traverse(node->left,depth+1);
        traverse(node->right,depth+1);
    }
    int maxDepth(TreeNode* root) {
        traverse(root,1);
        return this->max;
    }
};
