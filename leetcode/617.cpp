/*
Question:
You are given two binary trees root1 and root2.
Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not.
You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. 
Otherwise, the NOT null node will be used as the node of the new tree.
Return the merged tree.
Note: The merging process must start from the root nodes of both trees.
*/

// Approach:
// 1) If both nodes exists we set return a new node with it's value equal to their sum and then recursively call the function on the left and right nodes
// 2) If only one node exists then we return that node
// 3) If no nodes exists we return nullptr

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
    TreeNode* mergeTrees(TreeNode* root1, TreeNode* root2) {
        if(root1 && root2) {
            TreeNode *root = new TreeNode(root1->val + root2->val);
            root->left = mergeTrees(root1->left,root2->left);
            root->right = mergeTrees(root1->right,root2->right);
            return root;
        } 
        else if (root1) return root1;
        else if (root2) return root2; 
        return nullptr;
    }
};
