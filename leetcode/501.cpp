/*
Question:
Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.
If the tree has more than one mode, return them in any order.
Assume a BST is defined as follows:
    The left subtree of a node contains only nodes with keys less than or equal to the node's key.
    The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
    Both the left and right subtrees must also be binary search trees.
*/

// Approach:
// 1) Make an unordered map to track frequency of each value
// 2) Make a function traverse() to traverse the tree and update the key in the unordered_map with the value
//    Traverse the tree using traverse() and store the frequency of each value in the freq map
// 3) Loop over the map to find out the max frequency
// 4) Make a result vector res and loop over the freq map again and add all elements with the frequency
//    equal to the max frequency to the vector
// 5) Return the res vector

#include<unordered_map>

class Solution {
public:
    vector<int> findMode(TreeNode* root) {
        std::unordered_map<int,int> freq;
        traverse(root, freq);
        int maxFreq = 0;
        for(auto f: freq) {
            maxFreq = max(maxFreq, f.second);
        }
        vector<int> res;
        for(auto f: freq) {
            if(f.second == maxFreq) res.push_back(f.first);
        }
        return res;
    }
    void traverse(TreeNode *node, std::unordered_map<int,int> &freq) {
        if(!node)return;
        freq[node->val]++;
        traverse(node->left, freq);
        traverse(node->right, freq);
    }
};
