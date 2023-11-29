/*
Question:
Write a function that takes the binary representation of an unsigned integer and 
returns the number of '1' bits it has (also known as the Hamming weight).
Note:
    Note that in some languages, such as Java, there is no unsigned integer type. 
    In this case, the input will be given as a signed integer type. 
    It should not affect your implementation, as the integer's internal binary representation is the same, 
    whether it is signed or unsigned.
    In Java, the compiler represents the signed integers using 2's complement notation. 
    Therefore, in Example 3, the input represents the signed integer. -3.
*/

// Approach 1:
// 1) If n is 0 return 0
// 2) Otherwise recursively call hammingWeight on n/2 and add 1 if the current n is odd

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        if n == 0 {0}
        else { Self::hammingWeight(n/2) + if n%2 == 1 {1} else {0} }
    }
}

// Approach 2:
// 1) We convert the number to a binary string
// 2) Then we filter it to keep only 1 bits
// 3) Then we count and return the number of elements

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        format!("{:b}",n).chars().filter(|x| *x == '1').count() as i32
    }
}
