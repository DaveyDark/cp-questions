/*
Question:
An array arr a mountain if the following properties hold:

    arr.length >= 3
    There exists some i with 0 < i < arr.length - 1 such that:
        arr[0] < arr[1] < ... < arr[i - 1] < arr[i] 
        arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

Given a mountain array arr, return the index i such that arr[0] < arr[1] < ... < arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1].

You must solve it in O(log(arr.length)) time complexity.
*/

// Approach:
// Use binary search and at each mid point, check weather the array is ascending or descending
// 1) Make variables for left, right and mid of the binary search area
// 2) Loop until left < right
//    Calculate the mid, last to mid and next to mid element
//    If left < mid < right then array is ascending and we need to look in the right half for the peak, so set left to mid
//    If right > mid > left then array is descending and we need to look in the left half for the peak, so set right to mid
//    Otherwise if left < mid > right then we have found the peak so we break out
// 3) Return mid

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left,mut right) = (0, arr.len() as i32-1);
        let mut mid: i32 = 0;
        while left < right {
            mid = left + (right-left)/2;
            let last = *arr.get((mid- 1) as usize).unwrap_or(&arr[mid as usize]);
            let next = *arr.get((mid+1) as usize).unwrap_or(&arr[mid as usize]);
            if arr[mid as usize] >= last && arr[mid as usize] <= next {
                left = mid;
            }
            else if arr[mid as usize] <= last && arr[mid as usize] >= next {
                right = mid;
            } else {
                break;
            }
        }
        mid
    }
}
