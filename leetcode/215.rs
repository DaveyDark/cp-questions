/*
Question:
Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

Can you solve it without sorting?
*/

// Approach:
// We can use quickselect to do this
// Note: Rust has built in quickselect function for vector called select_nth_unstable() which gives best runtime and memory, 
// but here I used implemented it myself for learning
// 1) Make the quick select function which takes a slice and an intk
//    start by picking a randomized pivot index and initializing 3 vectors - left,mid and right
// 2) Then go through the given slice and add elements less than pivot to left, equal to pivot to mid and more than pivot to right
// 3) If right contains > k elements, we call quickselect on the right slice with the same k
//    If k > mid and right lengths combined, we call quickselect on the left slice with k - mid.len() - right.len()
//    Otherwise we return the pivot element
// 4) In the find_kth_largest function, we just call quickselect with the slice of the given array and k and return the returned value


use rand::Rng;

impl Solution {
    fn quickselect(nums: &[i32], k: i32) -> i32 {
        let pivot = rand::thread_rng().gen_range(0,nums.len());
        let mut left = vec!();
        let mut mid = vec!();
        let mut right = vec!();
        for &num in nums {
            if num < nums[pivot] { left.push(num) }
            else if num > nums[pivot] { right.push(num) }
            else { mid.push(num) }
        }
        if right.len() as i32 >= k {return Self::quickselect(&right[..], k)}
        else if mid.len() + right.len() < k as usize {
            return Self::quickselect(&left[..],k - mid.len() as i32 - right.len() as i32)
        }
        nums[pivot]
    }
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        Self::quickselect(&nums[..], k)
    }
}
