/*
Question:
Reverse bits of a given 32 bits unsigned integer.
*/

// Approach:
// 1) We convert the number to a binary string of a fixed length of 32 chars, extra space is covered by 0s inserted to the left
// 2) Then we reverse this string and pass it to from_str_radix which converts it into a number of the specified base, which in this case is 2
// 3) Then we return unwrapped the value we get from from_str_radix

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        u32::from_str_radix(&format!("{:0>32b}",x).chars().rev().collect::<String>(),2).unwrap()
    }
}
