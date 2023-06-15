/*
Question:
Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
*/

// Approach:
// 1) We first traverse the tree in level order and store the traversal in a 2d vector
// 2) Then we go over each row of the vector and calculate it's sum,
//    Then check if it's more than the current max, and set max to it if it is
// 3) We return the max sum at the end

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
    void traverse(TreeNode *node,int depth, vector<vector<int>> *vec){
        if(!node)return;
        if(depth >= vec->size()) vec->resize(depth+1);
        vec->at(depth).push_back(node->val);
        traverse(node->left,depth+1,vec);
        traverse(node->right,depth+1,vec);
    }
    int maxLevelSum(TreeNode* root) {
        vector<vector<int>> vec;
        traverse(root, 0, &vec);
        int level = 0;
        int max = INT_MIN;
        for(int i=0; i<vec.size(); i++) {
            int sum = std::accumulate(vec[i].begin(), vec[i].end(), 0);
            if(sum>max){
                level=i;
                max = sum;
            }
        }
        return level+1;
    }
};
