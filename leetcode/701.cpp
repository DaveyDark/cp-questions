/*
Question:
You are given the root node of a binary search tree (BST) and a value to insert into the tree. 
Return the root node of the BST after the insertion. 
It is guaranteed that the new value does not exist in the original BST.
Notice that there may exist multiple valid ways for the insertion, as long as the tree remains a BST after insertion. 
You can return any of them.
*/

// Approach:
// We traverse the tree comparing it to each node to find where the node should be and put the node then put it there

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
    void insert(TreeNode *root,int val){
        if(!root)return;
        if(val < root->val){
            if(root->left)insertIntoBST(root->left,val);
            else root->left = new TreeNode(val);
        } else {
            if(root->right)insertIntoBST(root->right,val);
            else root->right = new TreeNode(val);
        }
    }
    TreeNode* insertIntoBST(TreeNode* root, int val) {
        if(!root)return new TreeNode(val);
        insert(root,val);
        return root;
    }
};
