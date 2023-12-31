/*
Question:
A decimal number is called deci-binary if each of its digits is either 0 or 1 without any leading zeros.
For example, 101 and 1100 are deci-binary, while 112 and 3001 are not.

Given a string n that represents a positive decimal integer,

return the minimum number of positive deci-binary numbers needed so that they sum up to n.
*/

// Approach:
// 1) Iterate over the string and find the max digit.
// 2) Convert the digit to i32 and return it.

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        n.chars().max().unwrap().to_digit(10).unwrap() as i32
    }
}
