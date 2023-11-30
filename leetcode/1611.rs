/*
Question:
Given an integer n, you must transform it into 0 using the following operations any number of times:
    Change the rightmost (0th) bit in the binary representation of n.
    Change the ith bit in the binary representation of n if the (i-1)th bit 
    is set to 1 and the (i-2)th through 0th bits are set to 0.

Return the minimum number of operations to transform n into 0.
*/

// Approach:
// After typing out the starting few numbers for the test cases, you can observe that we basically
// have to trate the binary of the given number as gray code and convert it to binary
// 1) Copy the number to use as a bitmask
// 2) Loop until the mask is 0, right shift it by 1 bit and XOR n with the mask each iteration 
// 3) Return n

impl Solution {
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        let mut mask = n;
        while mask > 0 {
            mask >>= 1;
            n ^= mask;
        }
        n
    }
}
