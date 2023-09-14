/*
Question:
Given 3 positives numbers a, b and c. Return the minimum flips required in some bits of a and b to make ( a OR b == c ). (bitwise OR operation).
Flip operation consists of change any single bit 1 to 0 or change the bit 0 to 1 in their binary representation.
*/

// Approach:
// 1) Make a counter to count the number of flips needed
// 2) Loop until any one of the 3 given variables are > 0
//     Check if the OR of the last bits of A and B is equal to the last bit of C
//      If it isnt then check of the lats bit of C is 1, if it is then add 1 to flips
//      Otherwise add the sum of last bits of A and B to flips
//     Right shift all numbers by 1 bit
// 3) Return flips

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut flips = 0;
        while a > 0 || b > 0 || c > 0 {
            if (a & 1) | (b & 1) != (c & 1) {
                if (c & 1) == 1 {
                    flips += 1;
                } else {
                    flips += (b & 1) + (a & 1);
                }
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        flips
    }
}
