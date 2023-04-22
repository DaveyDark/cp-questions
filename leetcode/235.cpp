/*
Question:
Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
According to the definition of LCA on Wikipedia: 
“The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants 
(where we allow a node to be a descendant of itself).”
*/

// Approach:
// 1) We get references to the left and risht side of the node after calling the lowestCommonAncestor method on them
// 2) The function first checks if the left and right of the node are valid(i.e. return valid value) or if the current node is p or q, and returns the node if any of these is true
// 3) Then if only the left or only the right side is valid, we return the result of that side
// 4) otherwise we return nullptr at the end if we are unable to find the requested nodes in the tree

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */

class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* node, TreeNode* p, TreeNode* q) {
        if(!node)return node;
        TreeNode *left = lowestCommonAncestor(node->left,p,q);
        TreeNode *right = lowestCommonAncestor(node->right,p,q);
        if(left && right) return node;
        else if (node == p || node == q) return node;
        else if(left) return left;
        else if(right) return right;
        return nullptr;
    }
};
