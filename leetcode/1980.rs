/*
Question:
Given an array of strings nums containing n unique binary strings each of length n, 
return a binary string of length n that does not appear in nums. 
If there are multiple answers, you may return any of them.
*/

// Approach:
// 1) Check the width of the string and store it
// 2) Convert the string vector to a i32 vector of base 10 numbers
// 3) Sort the numbers
// 4) Starting from 0, count up until you find a number not in the vector
// 5) Format the string to a binary string of specified width and return it

impl Solution {
    pub fn find_different_binary_string(mut nums: Vec<String>) -> String {
        let width = nums[0].len();
        let mut nums: Vec<i32> = nums.iter().map(|n| i32::from_str_radix(n,2).unwrap()).collect();
        nums.sort_unstable();
        let mut i = 0;
        for n in &nums {
            if *n != i {break}
            i += 1;
        }
        format!("{:0>width$b}", i, width=width)
    }
}
