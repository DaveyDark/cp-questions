/*
Question:
Given the root of a binary tree,
find the maximum value v for which there exist different nodes a and b
where v = |a.val - b.val| and a is an ancestor of b.

A node a is an ancestor of b if either:
any child of a is equal to b or any child of a is an ancestor of b.
 */

// Approach:
// 1) Make a recursive function maxDiff that takes the current node, the min and
// max value of the ancestors and the max difference found so far. 2) In the
// function, update the min and max values by comparing with the current node
// value.
//    Then, update the max difference by comparing with the difference between
//    the max and min values. Then recursively call the function for the left
//    and right child.
// 3) Return the value of diff after calling maxDiff for the root node.

class Solution {
public:
  void maxDiff(TreeNode *root, int min, int max, int &diff) {
    if (!root)
      return;
    min = std::min(root->val, min);
    max = std::max(root->val, max);
    diff = std::max(diff, max - min);
    maxDiff(root->left, min, max, diff);
    maxDiff(root->right, min, max, diff);
  }
  int maxAncestorDiff(TreeNode *root) {
    int diff = 0;
    maxDiff(root, root->val, root->val, diff);
    return diff;
  }
};
