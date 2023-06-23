/*
Question:
Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.334.r
*/

// Approach:
// If we are moving from the start to end of the vector, every new index will be greater than any index we have already traversed
// So we will keep track of the least number's index we have traversed in first
// And also the second least number's index in second
// then lastly in the iteration we check if the current number is bigger than second, which means it's greater than first too
// and since it came later than first and second, it's index must also be greater, so we return true in this case
//
// Otherwise we jsut return false if we reach out of the loop

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first,mut second) = (i32::MAX,i32::MAX);
        for n in nums {
            if n < first { first = n }
            if n < second && first < n { second = n }
            if second < n { return true }
        }
        false
    }
}
