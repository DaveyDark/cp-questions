/*
Question:
Given the root of a binary tree, return the leftmost value in the last row of the tree.
*/

// Approach:
// We use level order traversal to traverse through the tree. 
// We keep track of the leftmost node in the current level and return it at the end.

class Solution {
  public int findBottomLeftValue(TreeNode root) {
    // Create a list to store the current level of nodes
    List<TreeNode> curr = new ArrayList<>();
    // Create a variable to store the result
    TreeNode res = null;

    // Add the root to the list
    curr.add(root);
    while(!curr.isEmpty()) {
      // List to store the next level of nodes
      List<TreeNode> next = new ArrayList<>();

      // Set the result to the first node in the list(leftmost node in the current level)
      res = curr.get(0);
      for(TreeNode n: curr) {
        // Add children to the next list if they exist
        if(n.left != null) next.add(n.left);
        if(n.right != null) next.add(n.right);
      }

      // Set the current list to the next list
      curr = next;
    }

    // Return the value of the leftmost node in the last level
    return res.val;
  }
}
