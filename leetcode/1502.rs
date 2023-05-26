/*
Question:
A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.
Given an array of numbers arr, return true if the array can be rearranged to form an arithmetic progression. Otherwise, return false.
*/

// Approach:
// 1) We calculate the common difference of the AP by subracting the first two elements
// 2) Then we go thorugh the rest of the elements in the array
//    If at any point the difference between adjacent elements is not equal to the common difference, we return false
// 3) Otherwise we return true at the end

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort();
        let mut diff = arr[1] - arr[0];
        for i in 1..arr.len() {
            if arr[i] - arr[i-1] != diff {return false;}
        }
        true
    }
}
