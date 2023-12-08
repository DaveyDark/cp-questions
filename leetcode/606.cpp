/*
Question:
Given the root of a binary tree, construct a string consisting of parenthesis 
and integers from a binary tree with the preorder traversal way, and return it.

Omit all the empty parenthesis pairs that do not affect the one-to-one mapping 
relationship between the string and the original binary tree.
*/

// Approach:
// 1) If root is null we return an empty string
// 2) Otherwise create a string and add the node value to it
// 3) If there is a left node, we append some parathesis and call tree2str recursively
//    on the left child. If there is no left child but a right child, append ()
// 4) If there is a right child, append parathesis and call tree2str on the right child
// 5) Return the string

class Solution {
public:
    string tree2str(TreeNode* root) {
        if(!root) return "";
        string st;
        st += to_string(root->val);
        if(root->left) {
            st += "(";
            st += tree2str(root->left);
            st += ")";
        } else if(root->right) st += "()";
        if(root->right) {
            st += "(";
            st += tree2str(root->right);
            st += ")";
        }
        return st;
    }
};
