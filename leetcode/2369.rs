/*
Question:
You are given a 0-indexed integer array nums. You have to partition the array into one or more contiguous subarrays.

We call a partition of the array valid if each of the obtained subarrays satisfies one of the following conditions:

    The subarray consists of exactly 2 equal elements. For example, the subarray [2,2] is good.
    The subarray consists of exactly 3 equal elements. For example, the subarray [4,4,4] is good.
    The subarray consists of exactly 3 consecutive increasing elements, that is, the difference between adjacent elements is 1. For example, the subarray [3,4,5] is good, but the subarray [1,3,5] is not.

Return true if the array has at least one valid partition. Otherwise, return false.
*/

// Approach:
// 1) Make a function check() that will check a given array slice for the 3 conditions given in the problem and return the result
// 2) Make a vector the size of nums for dp, where dp[i] will represent if the subarray nums[0..1] can be validly partitioned
// 3) Loop through nums starting fromt the second index(since a subarray of size 1 cannot be valid)
//    For a given range 0..i, if the last 3 or last 2 elements are a valid partition and dp[i-2] or dp[i-3] is true respectively, 
//    we can say that that slice is a valid partion and dp[i] is true
// 4) At the end we can return the last element of the dp vector

impl Solution {
    fn check(nums: &[i32]) -> bool {
        (nums.len() == 2 && nums[0] == nums[1]) 
        || (nums.len() == 3 && nums[0] == nums[1] && nums[1] == nums[2]) 
        || (nums.len() == 3 && nums[0]+1 == nums[1] && nums[1]+1 == nums[2])
    }
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp = vec!(false; nums.len());
        for i in 1..nums.len() {
            if Self::check(&nums[i-1..=i]) && *dp.get(i-2).unwrap_or(&true) { dp[i] = true }
            else if i > 1 && Self::check(&nums[i-2..=i]) && *dp.get(i-3).unwrap_or(&true) { dp[i] = true }
        }
        dp[nums.len() - 1]
    }
}
