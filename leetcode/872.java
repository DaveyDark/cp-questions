/*
Question:
Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.

For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).

Two binary trees are considered leaf-similar if their leaf value sequence is the same.

Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
*/

// Approach:
// Get the leaf list of both trees and compare them and return the result.
//
// To get the leaf list, make a recursive function leafList.
// If the root is null, return null.
// If the root is a leaf, return a list with the root value.
// Else, return the list of left subtree and right subtree.

class Solution {
  ArrayList<Integer> leafList(TreeNode root) {
    if(root == null) {
      return null;
    }
    ArrayList<Integer> res = new ArrayList<>();
    if(root.left == null && root.right == null) {
      res.add(root.val);
    } else {
      if(root.left != null) res.addAll(leafList(root.left));
      if(root.right != null) res.addAll(leafList(root.right));
    }
    return res;
  }
  public boolean leafSimilar(TreeNode root1, TreeNode root2) {
    return leafList(root1).equals(leafList(root2));
  }
}
