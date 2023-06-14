/*
Question:
Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.
*/

// Approach:
// 1) We first traverse the given tree inorfer, which returns a sorted vector of the elements since it is a binary search tree
// 2) Then we go over the vector and subtract adjacent elements, and store the minimum difference in min_diff
// 3) Then we return min_diff at the end

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
    void traverse(TreeNode *node, std::vector<int> *vec){
        if(!node)return;
        if(node->left)traverse(node->left,vec);
        vec->push_back(node->val);
        if(node->right)traverse(node->right,vec);
    }
    int getMinimumDifference(TreeNode* root) {
        std::vector<int> vec;
        traverse(root,&vec);
        int min_diff = INT_MAX;
        for (int i = 1; i < vec.size(); i++)
            min_diff = min(vec[i] - vec[i-1], min_diff);
        return min_diff;
    }
};
