/*
Question:
Given the root node of a binary search tree and two integers low and high,
return the sum of values of all nodes with a value in the inclusive range [low,
high].
*/

// Approach:
// Use recursion to traverse the tree and add the value if it is in the range.
// 1) If the root is null, return 0.
// 2) If the root is in the range, add the value and recurse on both left and
// right. 3) If the root is greater than high, recurse on left. 4) Else recurse
// on right.

class Solution {
public:
  int rangeSumBST(TreeNode *root, int low, int high) {
    if (!root)
      return 0;
    if (root->val >= low && root->val <= high) {
      int sum = 0;
      return root->val + rangeSumBST(root->left, low, high) +
             rangeSumBST(root->right, low, high);
    } else if (root->val > high) {
      return rangeSumBST(root->left, low, high);
    } else {
      return rangeSumBST(root->right, low, high);
    }
  }
};
