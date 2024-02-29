/*
Question:
A binary tree is named Even-Odd if it meets the following conditions:
  The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
  For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
  For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).

Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.
*/

// Approach:
// Perform level order traversal and check the conditions for the odd or even row

class Solution {
  public boolean isEvenOddTree(TreeNode root) {
  // Create a list to store the current level of nodes
  List<TreeNode> curr = new ArrayList<>();
  // Create a variable to store if the row is even or odd
  boolean even = true;

  // Add the root to the list
  curr.add(root);
  while(!curr.isEmpty()) {
    // List to store the next level of nodes
    List<TreeNode> next = new ArrayList<>();
    
    // Variable to track the last node
    TreeNode last = null;
    for(int i = 0; i < curr.size(); i++) {
      // Check the conditions for the odd or even row
      if(i != 0 && even && last.val >= curr.get(i).val) 
        return false;
      if(i != 0 && !even && last.val <= curr.get(i).val) 
        return false;
      if(even && curr.get(i).val%2 == 0) return false;
      else if(!even && curr.get(i).val%2 == 1) return false;

      // Add children to the next list if they exist
      if(curr.get(i).left != null) next.add(curr.get(i).left);
      if(curr.get(i).right != null) next.add(curr.get(i).right);
      last = curr.get(i);
    }

    // Set the current list to the next list
    curr = next;
    even = !even;
  }

  // Return true if we reached the end
  return true;
  }
}
