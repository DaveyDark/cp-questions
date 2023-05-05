/*
Question:
You are a professional robber planning to rob houses along a street. 
Each house has a certain amount of money stashed, 
the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected 
and it will automatically contact the police if two adjacent houses were broken into on the same night.
Given an integer array nums representing the amount of money of each house, 
return the maximum amount of money you can rob tonight without alerting the police.
*/

// Approach:
// 1) First we check the edge case of the list being empty and return 0 if it is
// 2) We keep track of the last two houses robbed as we loop through all the houses
// 3) Then we calculate the value of prev1 which will be the max value of prev2 + n and prev1
// 4) Then we give prev2 the previous value of prev1
// 5) We return prev1 at then end of the function

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {return 0;}
        let mut prev1 = 0;
        let mut prev2 = 0;
        for n in &nums {
            let tmp = prev1;
            prev1 = i32::max(prev2 + n,prev1);
            prev2 = tmp;
        }
        prev1
    }
}
