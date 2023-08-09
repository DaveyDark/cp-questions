/*
Question:
You are given a positive integer n. Each digit of n has a sign according to the following rules:

    The most significant digit is assigned a positive sign.
    Each other digit has an opposite sign to its adjacent digits.

Return the sum of all digits with their corresponding sign.
*/

// Approach:
// 1) Covert the number to string and enumerate over it's chars
// 2) If the enumeration index is even, add the digit, otherwise subtract the digit
// 3) Return the sum at the end

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut sum = 0;
        for (i,ch) in n.to_string().chars().enumerate() {
            if i%2 == 0 {sum += ch.to_digit(10).unwrap() as i32}
            else {sum -= ch.to_digit(10).unwrap() as i32}
        }
        sum
    }
}
