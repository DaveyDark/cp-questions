/*
Question:
Given two non-negative integers low and high. Return the count of odd numbers between low and high (inclusive).
*/

// Approach
// 1) First we calculate n, the number of elements in the range
// 2) Then if it is even, then the number of odd numbers will just be n/1
// 3) If it's odd, then we have to check if the first number is even,
//    If it is then the number of odd numbers is n/2 again
//    If it isn't then it's n/2 + 1

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let n = (high - low + 1);
        if n%2 == 0 { n/2 } 
        else {
            if(low%2==0) { n/2 } 
            else { (n/2)+1 }
        }
    }
}
