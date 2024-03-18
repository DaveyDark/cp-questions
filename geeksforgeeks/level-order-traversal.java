/*
Question:
Given a root of a binary tree with n nodes, find its level order traversal.
Level order traversal of a tree is breadth-first traversal for the tree.
*/

// Approach:
// We use a queue to store the nodes of the tree
// Initially, we add the root node to the queue
//
// Then until the queue is empty, we add the current node to the traversal list
// And then enqueue the left and right children of the current node if they exist
//
// Finally, we return the traversal list

class Solution
{
  static ArrayList <Integer> levelOrder(Node root) 
  {
    ArrayList<Integer> traversal = new ArrayList<>();
    Queue<Node> queue = new ArrayDeque<>();
    
    queue.offer(root);
    
    while(!queue.isEmpty()) {
      Node n = queue.poll();
      traversal.add(n.data);
      if(n.left != null) queue.offer(n.left);
      if(n.right != null) queue.offer(n.right);
    }
    
    return traversal;
  }
}
