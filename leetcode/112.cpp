/*
Question:
Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path 
such that adding up all the values along the path equals targetSum.
A leaf is a node with no children.
*/

// Approach:
// 1) We traverse the tree keeping track of sum of nodes leading to current node
// 2) If the current node is a leaf node then we check if the sum matches the target and return true if it does
// 3) Otherwise we traverse the children of the current node

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
    bool traverse(TreeNode *node,int sum,int target){
        if(!node)return false;
        if(!node->left && !node->right){
            // leaf node
            if (sum + node->val == target) {
                return true;
            }
        }
        return traverse(node->left,sum + node->val,target) || traverse(node->right,sum + node->val,target);
    }
    bool hasPathSum(TreeNode* root, int targetSum) {
        return traverse(root,0,targetSum);
    }
};
