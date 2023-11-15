/*
Question:
You are given an array of positive integers arr. Perform some operations (possibly none) on arr so that it satisfies these conditions:
    The value of the first element in arr must be 1.
    The absolute difference between any 2 adjacent elements must be less than or equal to 1. In other words, abs(arr[i] - arr[i - 1]) <= 1 for each i where 1 <= i < arr.length (0-indexed). abs(x) is the absolute value of x.

There are 2 types of operations that you can perform any number of times:
    Decrease the value of any element of arr to a smaller positive integer.
    Rearrange the elements of arr to be in any order.

Return the maximum possible value of an element in arr after performing the operations to satisfy the conditions.
*/

// Approach:
// 1) Sort the array
// 2) Loop over the array keeping track of the last element and a counter to store the max element
//      Each iteration if the current element isn't equal to the last, set last to last+1 and
//      increment max
// 3) Return max

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();
        let mut max = 0;
        let mut last = 0;
        for n in &arr {
            if *n != last {
                last = last+1;
                max += 1;
            }
        }
        max
    }
}
