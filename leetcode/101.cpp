/*
Question:
Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
*/

// Approach:
// If the tree is symmetic then the left node of left side will be equal to the right node of right side and vice versa
// We just check that for each node in the tree

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
    bool isEquivalent(TreeNode* left, TreeNode* right) {
        if (!left || !right) return left == right;
        else return left->val == right->val && isEquivalent(left->left, right->right) && isEquivalent(right->left, left->right);
    }
    bool isSymmetric(TreeNode* root) {
        return isEquivalent(root->left, root->right);
    }
};
