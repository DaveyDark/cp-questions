/*
Question:
Given an integer n, return true if it is a power of two. Otherwise, return false.
An integer n is a power of two, if there exists an integer x such that n == 2x.
*/

// Approach:
// If the binary form of the number only has a single 1 bit then it is a power of two
// 1) We first check if the number is positive as a negetive number is not a power of two
// 2) Then we convert the number to a binary string
// 3) Then we filter the string to keep only 1 bits
// 4) Then we check if there is exactly 1 positive bit and if there is then we return true, otherwise false

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n > 0 && format!("{:b}",n).chars().filter(|x| *x == '1').count() == 1 {true} else {false}
    }
}
