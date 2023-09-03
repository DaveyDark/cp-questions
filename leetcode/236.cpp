/*
Question:
Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.

According to the definition of LCA on Wikipedia: 
“The lowest common ancestor is defined between two nodes p and q as the lowest node in T 
that has both p and q as descendants (where we allow a node to be a descendant of itself).”
*/

// Approach:
// Make the function recursive and use DFS to locate the lowestCommonAncestor
// 1) If the root is null we return null(root)
// 2) Call lowestCommonAncestor on the left and right children
// 3) If both children returned non-null i.e. both have either p or q, we return the current node as the LCA
//    Otherwise if the current node is one of p or q, we also return it
// 4) Otherwise, if one of l or r is not null, we return it
// 5) Otherwise we just return nullptr since neither the children nor the current node are p or q

class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q, bool found = false) {
        if(!root)return root;
        TreeNode *l = lowestCommonAncestor(root->left,p,q);
        TreeNode *r = lowestCommonAncestor(root->right,p,q);
        if(l && r) return root;
        if(root == p || root == q) return root;
        else if(l) return l;
        else if(r) return r;
        else return nullptr;
    }
};
