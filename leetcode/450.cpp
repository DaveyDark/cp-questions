/*
Question:
Given a root node reference of a BST and a key, delete the node with the given
key in the BST. Return the root node reference (possibly updated) of the BST.

Basically, the deletion can be divided into two stages:
  Search for a node to remove.
  If the node is found, delete the node.
*/

// Approach:
// 1) If the root is null, return null.
// 2) If the key is less than root->val, delete the key from the left subtree.
// 3) If the key is greater than root->val, delete the key from the right
// subtree. 4) If the key is equal to root->val, then
//    a) If the root has no children, set root to null.
//    b) If the root has only one child, set root to that child.
//    c) If the root has both children, find the inorder successor of the root.
//       Copy the value of the inorder successor to the root.
//       Delete the inorder successor from the left subtree.
// 5) Return root.

class Solution {
public:
  TreeNode *deleteNode(TreeNode *root, int key) {
    if (!root)
      return nullptr;
    if (key < root->val)
      root->left = deleteNode(root->left, key);
    else if (key > root->val)
      root->right = deleteNode(root->right, key);
    else {
      if (!root->left && !root->right) {
        root = nullptr;
      } else if (root->left) {
        if (root->right) {
          TreeNode *temp = root->left;
          while (temp->right != NULL)
            temp = temp->right;
          root->val = temp->val;
          root->left = deleteNode(root->left, temp->val);
        } else {
          root = root->left;
        }
      } else {
        root = root->right;
      }
    }
    return root;
  }
};
