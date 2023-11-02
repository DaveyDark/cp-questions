/*
Question:
Given the root of a binary tree, return the number of nodes where the value of the node is equal to the average of the values in its subtree.

Note:
    The average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
    A subtree of root is a tree consisting of root and all of its descendants.
*/

// Approach:
// 1) Make a recursive function traverse() to traverse the tree while keeping track of the sum and count of the nodes in the subtree
//    Return those values as a pair where the sum is the first value and the count of nodes is the second
//    Add the sum and count of the left and right subtrees to the node value by calling traverse() on the left and right nodes
//    Then return the final result pair
// 2) Init a counter and call the traverse() function on the root node
// 3) Return the counter

class Solution {
public:
    int averageOfSubtree(TreeNode* root) {
        int ctr = 0;
        traverse(root, ctr);
        return ctr;
    }
    std::pair<long int,int> traverse(TreeNode *node, int &ctr) {
        if(!node) return std::pair<long int,int>(0,0);
        std::pair<long int, int> leftPair = traverse(node->left, ctr);
        std::pair<long int, int> rightPair = traverse(node->right, ctr);
        std::pair<long int, int> res(leftPair.first + node->val + rightPair.first, leftPair.second + rightPair.second + 1);
        if(res.second > 0 && node->val == (res.first/res.second)) ctr++;
        return res;
    }
};
