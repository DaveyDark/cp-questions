/*
Question:
Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.

For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).

Two binary trees are considered leaf-similar if their leaf value sequence is the same.

Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
*/

// Approach:
// 1) Make a method traverse that traverses the tree, and takes a vector and add the node to that vector if it's a leaf nodes
//    traverse() should thus return a vector of leaf nodes from left to right
// 2) Make two vectors to store the leaf nodes to both trees and populate them using traverse()
// 3) Compare the 2 vectors and return true if they are equal

class Solution {
public:
    void traverse(TreeNode *node, vector<int> *leaves) {
        if(!node) return;
        if(!node->left && !node->right)leaves->push_back(node->val);
        if(node->left) traverse(node->left,leaves);
        if(node->right) traverse(node->right,leaves);
    }
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        vector<int> leaves1, leaves2;
        traverse(root1, &leaves1);
        traverse(root2, &leaves2);
        return leaves1 == leaves2;
    }
};
