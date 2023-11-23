/*
Question:
A sequence of numbers is called arithmetic if it consists of at least two elements, 
and the difference between every two consecutive elements is the same. 
More formally, a sequence s is arithmetic if and only if s[i+1] - s[i] == s[1] - s[0] for all valid i.

For example, these are arithmetic sequences:
1, 3, 5, 7, 9
7, 7, 7, 7
3, -1, -5, -9

The following sequence is not arithmetic:

1, 1, 2, 5, 7

You are given an array of n integers, nums, and two arrays of m integers each, l and r, representing the m range queries, where the ith query is the range [l[i], r[i]]. All the arrays are 0-indexed.

Return a list of boolean elements answer, 
where answer[i] is true if the subarray nums[l[i]], nums[l[i]+1], ... , nums[r[i]] 
can be rearranged to form an arithmetic sequence, and false otherwise.
*/

// Approach:
// 1) Make a vector to store the result
// 2) Loop over the given left and right indices
//      Make a cloned subarray of the given range and sort it
//      Store the difference of the first two elements in a variable and then loop over the
//      remaining pairs and verify that their difference is equal. If it isn't then the current
//      subarray isn't a AP.
// 3) Return the res array

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut res = vec!();
        for (left,right) in l.iter().zip(r.iter()) {
            let mut subarr = nums[*left as usize..=*right as usize].iter().cloned().collect::<Vec<i32>>();
            subarr.sort_unstable();
            let diff = (subarr[1]-subarr[0]).abs();
            let mut ap = true;
            for i in 1..subarr.len()-1 {
                if subarr[i+1] - subarr[i] != diff {ap = false}
            }
            res.push(ap);
        }
        res
    }
}
