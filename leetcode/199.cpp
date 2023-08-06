/*
Question:
Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
*/

// Appraoch
// 1) Make a vector to store the view and return it as empty if the root is null.
// 2) Make a queue for BFS of the tree and push the root node onto it
// 3) Until the queue is empty, do BFS on the tree
//      Check the size of the current level and loop for that size
//        Every iteration, take out the front node of the queue and push it's children into the queue if they exist
//      After the inner loop, push the value of the last popped node to the view
// 4) Return the view vector at the end

class Solution {
public:
    vector<int> rightSideView(TreeNode* root) {
        vector<int> view;
        if(!root)return view;
        std::queue<TreeNode*> q;
        q.push(root);
        while(!q.empty()) {
            int levelSize = q.size();
            TreeNode *node;
            for(int i=0; i<levelSize; i++) {
                node = q.front();
                if(node->left)q.push(node->left);
                if(node->right)q.push(node->right);
                q.pop();
            }
            if(node)view.push_back(node->val);
        }
        return view;
    }
};
