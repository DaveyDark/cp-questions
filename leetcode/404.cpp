/*
Question:
Given the root of a binary tree, return the sum of all left leaves.

A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.
*/

// Approach:
// Use recursion to DFS the tree
// 1) If the root is null return 0
// 2) Otherwise call sumOfLeftLeaves on left and right children of the root and sum their return values
// 3) If the root node is a left child and a left node, return the value of root + sum, if not then just return sum

class Solution {
public:
    int sumOfLeftLeaves(TreeNode* root, bool isLeft = false) {
        if(!root) return 0;
        int sum = 0;
        sum += sumOfLeftLeaves(root->left,true);
        sum += sumOfLeftLeaves(root->right,false);
        return (isLeft && !root->left && !root->right)? sum+root->val: sum;
    }
};
