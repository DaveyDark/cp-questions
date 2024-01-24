/*
Question:
Given a binary tree where node values are digits from 1 to 9.
A path in the binary tree is said to be pseudo-palindromic if at least one
permutation of the node values in the path is a palindrome.

Return the number of pseudo-palindromic paths going from the root node to leaf
nodes.
*/

// Approach:
// 1) Use stack to do DFS
// 2) For each node, XOR the value of the node to the visited bitset to indicate
//   that we have encountered the node
// 3) If the node is a leaf, check if the
//   visited bitset has only one bit set and increment the count if it is
// 4) If
//   the node is not a leaf, push the children to the stack
// 5) Repeat until the
//   stack is empty
// 6) Return the count

class Solution {
public:
  int pseudoPalindromicPaths(TreeNode *root) {
    int visited = 0, cnt = 0;
    stack<std::pair<TreeNode *, int>> st;
    st.push({root, 0});

    while (!st.empty()) {
      std::pair<TreeNode *, int> p = st.top();
      st.pop();

      p.second ^= (1 << p.first->val);

      if (p.first->left || p.first->right) {
        if (p.first->right)
          st.push({p.first->right, p.second});
        if (p.first->left)
          st.push({p.first->left, p.second});
      } else {
        if ((p.second & (p.second - 1)) == 0)
          cnt++;
      }
    }

    return cnt;
  }
};
