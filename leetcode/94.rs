/*
Question:
Given the root of a binary tree, return the inorder traversal of its nodes' values.
*/

// Appraoch:
// 1) Create a ans vector
// 2) Match the root and call traverse() on it if it is Some() and pass the ans vector as a &mut
//      In traverse(), we borrow a reference to node and then check if the left child is present,
//      then call traverse() on the left if it is. Then we push teh current value to ans and then
//      do the same for the right node
// 3) Return the ans

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        match root {
            Some(r) => Self::traverse(r, &mut ans),
            None => (),
        }
        ans
    }
    fn traverse(node: Rc<RefCell<TreeNode>>, ans: &mut Vec<i32>) {
        let node = node.borrow();
        if let Some(left) = &node.left {
            Self::traverse(Rc::clone(&left), ans);    
        }
        ans.push(node.val);
        if let Some(right) = &node.right {
            Self::traverse(Rc::clone(&right), ans);    
        }
    }
}
