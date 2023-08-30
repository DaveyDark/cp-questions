/*
Question:
You are given the root of a binary tree.

A ZigZag path for a binary tree is defined as follow:
    Choose any node in the binary tree and a direction (right or left).
    If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
    Change the direction from right to left or from left to right.
    Repeat the second and third steps until you can't move in the tree.
Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).

Return the longest ZigZag path contained in that tree.
*/

// Approach:
// 1) Make recursive function zigZag to DFS the tree, in which
//    If the node is null we return the current distance
//    Otherwise we check if the left node was a right child
//    If it was then we call zigZag on the left with incremented streak and on right with the streak reset and return the max of the 2 values
//    Otherwise we call Zigzag  on the left with a reset streak and on the right with an incremented streak and return the max value
// 2) Call zigzag with the lastWasRight varaiable set to both values and return the max of the two values returned

class Solution {
public:
    int longestZigZag(TreeNode* root) {
        return max(zigZag(root->left, false), zigZag(root->right, true));
    }
    int zigZag(TreeNode* root, bool lastWasRight = false, int streak = 0) {
        if(!root) return streak;
        if(lastWasRight) 
            return max(zigZag(root->left, false, streak+1), zigZag(root->right, true));
        else 
            return max(zigZag(root->left, false), zigZag(root->right, true, streak+1));
    }
};
