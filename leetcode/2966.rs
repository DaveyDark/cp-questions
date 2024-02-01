/*
Question:
You are given an integer array nums of size n and a positive integer k.

Divide the array into one or more arrays of size 3 satisfying the following conditions:
    Each element of nums should be in exactly one array.
    The difference between any two elements in one array is less than or equal to k.

Return a 2D array containing all the arrays.
If it is impossible to satisfy the conditions, return an empty array.
And if there are multiple answers, return any of them.
*/

// Approach:
// 1) Sort the array
// 2) Iterate over the array in chunks of 3
// 3) Check the difference between the first and last element and return empty if it is greater than
//    Otherwise, push the chunk into the result vec
// 4) Return the result vec at the end

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res: Vec<Vec<i32>> = vec![];
        for chk in nums.chunks(3) {
            if (chk[2] - chk[0]).abs() > k {
                return vec![];
            }
            res.push(chk.to_vec());
        }
        res
    }
}
