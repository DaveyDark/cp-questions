/*
Question:
Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.
Return the number of good nodes in the binary tree.
*/

// Approach:
// We can use DFS with recursion to solve this
// 1) We add an additional parameter max with the default value of INT_MIN to represent the max value in current path
// 2) If the node is null we return 0;
// 3) If the node value is more than max, we call goodNodes on the children of then node and giving the node's value as max and all 1 to that and return it
//    Otherwise we just call goodNodes on the children while using the same max value

class Solution {
public:
    int goodNodes(TreeNode *node, int max=INT_MIN) {
        if(!node)return 0;
        if(node->val >= max) 
            return 1 + goodNodes(node->left,node->val) + goodNodes(node->right,node->val);
        return goodNodes(node->left,max) + goodNodes(node->right,max);
    }
};
