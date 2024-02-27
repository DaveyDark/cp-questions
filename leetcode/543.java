/*
Question:
Given the root of a binary tree, return the length of the diameter of the tree.

The diameter of a binary tree is the length of the longest path between any two nodes in a tree. 
This path may or may not pass through the root.

The length of a path between two nodes is represented by the number of edges between them.
*/

// Approach:
// The largest path will always have a node as the pivot and the path will be 
// the sum of the depth of the left and right subtree.
//
// We can brute force this by calculating the depth of each node and then calculating
// the diameter of the tree at each node and keep track of the max diameter so far.

class Solution {
  // Stores the max diameter so far
  private int ans = 0;
  public int diameterOfBinaryTree(TreeNode root) {
    if(root == null) return 0;
    traverse(root);
    return ans;
  }
  int traverse(TreeNode node) {
    if(node == null) return 0;
    // Calculate the depth of the left and right subtree
    int left = traverse(node.left);
    int right = traverse(node.right);

    // Update the max diameter so far
    ans = Math.max(ans, left + right);

    // Return the depth of the current node
    return 1 + Math.max(left, right);
  }
}
