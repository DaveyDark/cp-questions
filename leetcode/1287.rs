/*
Question:
Given an integer array sorted in non-decreasing order, there is exactly one integer in the array that occurs more than 25% of the time, return that integer.
*/

// Approach:
// 1) Iterate over windows of size n/4 + 1 which is about 25% of the array
// 2) If we find a window with all equal elements, we return the first element of that window
// 3) Return 0 if no valid element is found

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        for w in arr.windows((arr.len()/4)+1) {
            if !w.iter().any(|&x| x != w[0]) {
                return w[0] 
            }
        }
        0
    }
}
