/*
Question:
Given an array nums of distinct integers, return all the possible permutations. 
You can return the answer in any order.
*/

// Approach:
// The backtrack functions works as follows
// 1) The base case checls if the current index exceeds the length of the permutation
//    And pushes the current permutation to the ans vector if it does
// 2) Otherwise we loop from idx(current index) to the end of array
//    We swap the ith element with idx and then backtrack with the incremented index
//    After that we undo the swap and proceed to next iteration of the loop

impl Solution {
    fn backtrack(nums: &mut Vec<i32>, idx: usize, ans: &mut Vec<Vec<i32>>) {
        if idx >= nums.len() {
            ans.push(nums.clone());
            return;
        }

        for i in idx..nums.len() {
            nums.swap(i, idx);
            Self::backtrack(nums, idx + 1, ans);
            nums.swap(idx, i);
        }
    }
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        Self::backtrack(&mut nums, 0, &mut ans);
        ans
    }
}
