/*
Question:
Given the root of a binary tree, determine if it is a valid binary search tree (BST).
A valid BST is defined as follows:
    The left
    subtree
    of a node contains only nodes with keys less than the node's key.
    The right subtree of a node contains only nodes with keys greater than the node's key.
    Both the left and right subtrees must also be binary search trees.
*/

// Approach:
// 1) We do an inorder traversal of the tree and store it in a vector
// 2) If the vector is properly sorted i.e. each element is > it's previous elements, it is valid and we return true
// 3) Otherwise we return false

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
    void traverse(TreeNode *node,vector<int> *vec){
        if(!node)return;
        traverse(node->left,vec);
        vec->push_back(node->val);
        traverse(node->right,vec);
    }
    bool isValidBST(TreeNode* root) {
        vector<int> vec;
        traverse(root,&vec);
        int prev = vec[0];
        for(int i = 1; i<vec.size(); i++){
            if(prev>=vec[i])return false;
            prev = vec[i];
        }
        return true;
    }
};
