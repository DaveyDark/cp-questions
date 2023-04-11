/*
Question:
Given two integer arrays nums1 and nums2, return an array of their intersection. 
Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
*/

// Approach
// 1) We check which array is smaller and pass both the arrays to the calc_intersection function
//    This is so that the map is created for the smaller array to conserve memory
// 2) We make a hashmap and use it to calculate frequency of elements in the first array
// 3) Then we loop through the second array and add elements to the solution vector if they have a value > 1 in the hashmap
//    We also update the value for that key by -1 after adding it to the solution vector
// 4) The solution vector is returned

use std::collections::HashMap;

fn calc_intersection(nums1: &Vec<i32>,nums2: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut ans: Vec<i32> = Vec::new();
    for n in nums1 {
        map.insert(n,(*map.get(&n).unwrap_or(&0))+1);
    }
    for n in nums2 {
        match map.get_mut(&n) {
            Some(val) => {
                if *val > 0 {
                    ans.push(*n);
                    *val -= 1;
                }
            }
            None => {}
        }
    }
    ans
}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut intersection: Vec<i32> = Vec::new();
        if nums1.len() > nums2.len() {
            return calc_intersection(&nums2, &nums1);
        }
        return calc_intersection(&nums1, &nums2);
    }
}
