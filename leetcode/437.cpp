/*
Question:
Given the root of a binary tree and an integer targetSum,
return the number of paths where the sum of the values along the path equals
targetSum.

The path does not need to start or end at the root or a leaf,
but it must go downwards (i.e., traveling only from parent nodes to child
nodes).
*/

// Approach:
// 1) Make a counter and pass it by reference to the recursive function sumTree.
// 2) sumTree will recursively traverse the tree and call getSum on each node
// and add the result to the counter. 3) getSum will recursively traverse the
// tree and return the number of paths that sum to the target.

class Solution {
public:
  int pathSum(TreeNode *root, int targetSum) {
    int count = 0;
    sumTree(root, targetSum, count);
    return count;
  }
  void sumTree(TreeNode *node, int targetSum, int &count) {
    if (!node)
      return;
    count += getSum(node, targetSum);
    sumTree(node->left, targetSum, count);
    sumTree(node->right, targetSum, count);
  }
  int getSum(TreeNode *node, int target, long int sum = 0) {
    if (!node)
      return 0;
    sum += node->val;
    return (sum == target) + getSum(node->left, target, sum) +
           getSum(node->right, target, sum);
  }
};
