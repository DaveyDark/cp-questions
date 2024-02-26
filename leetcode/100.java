/*
Question:
Given the roots of two binary trees p and q, write a function to check if they are the same or not.

Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
*/

// Approach:
// We will do a DFS traversal of the tree and compare the nodes of the tree
// We will compare each node of one tree to the corresponding node of the other tree
// If any of them are unequal, we will return false
// Otherwise, we can return true

class Solution {
  public boolean isSameTree(TreeNode p, TreeNode q) {
    // Create a stack to hold the nodes of the tree
    Stack<Pair<TreeNode, TreeNode>> st = new Stack<>();

    // Insert the root nodes of the trees into the stack
    st.push(new Pair(p, q));

    while(!st.isEmpty()) {
      // Pop the nodes from the stack
      Pair<TreeNode, TreeNode> pr = st.pop();
      TreeNode n1 = pr.getKey();
      TreeNode n2 = pr.getValue();

      if(n1 == null || n2 == null) {
        // if only one of the nodes is null, return false
        if(n1 != n2) return false;
        // otherwise if both are null, continue
        continue;
      }

      // if the values of the nodes are not equal, return false
      if(n1.val != n2.val) return false;

      // push the left and right nodes of the trees into the stack
      st.push(new Pair(n1.left, n2.left));
      st.push(new Pair(n1.right, n2.right));
    }

    // if the stack is empty, return true
    return true;
  }
}
