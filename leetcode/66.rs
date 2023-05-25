/*
Question:
You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. 
The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
Increment the large integer by one and return the resulting array of digits.
*/

// Approach:
// 1) We init carry to 1
// 2) We go though the array in reverse order and add carry to each element,
//    Then calculate carry to digits[i]/10 
//    and digits[i] to digits[i]%10
// 3) If carry is still left after then loop then we append it to the start of the vector and return it

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            digits[i] += carry;
            carry = digits[i]/10;
            digits[i] %= 10;
        }
        if carry != 0 {digits.insert(0,carry)}
        digits
    }
}
