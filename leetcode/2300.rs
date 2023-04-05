/*
Question:
You are given two positive integer arrays spells and potions, of length n and m respectively, where 
spells[i] represents the strength of the ith spell and potions[j] represents the strength of the jth potion.

You are also given an integer success. A spell and potion pair is considered successful if the product of their strengths is at least success.

Return an integer array pairs of length n where pairs[i] is the number of potions that will form a successful pair with the ith spell.
*/

// Approach:
// 1) Loop through given spells
// 2) Find factor which is success rate / current spell. Any potion >= factor will be successful
// 3) Find the first element in the potions vector which is >= factor
// 4) Calculate the number of successful combinations by subtracting the index of that element from
//    the length of the vector


fn b_search(list: &Vec<i32>, key: i32) -> Option<usize> {
    //searches through the vector for first occurrence of an element >= key
    
    //edge case - when key < all elements
    if key <= list[0] {
        return Some(0);
    }

    let mut low = 0;
    let mut high = list.len();
    while low < high {
        let mid = low + (high - low) / 2;

        if list[mid] >= key {
            // Check if there is a previous occurrence that is less than key
            if mid > 0 && list[mid - 1] < key {
                return Some(mid);
            } else {
                high = mid;
            }
        } else {
            low = mid + 1;
        }
    }

    None
}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut pairs: Vec<i32> = Vec::new();
        let mut potions = potions.clone();
        potions.sort();
        for spell in &spells {
            let mut n: i32 = -1;
            let factor: i32 = (success as f64 / *spell as f64).ceil() as i32;
            //check if factor or any number > factor exists in vector
            match b_search(&potions, factor) {
                Some(index) => n = index as i32,
                None => n = -1,
            }
            if n >= 0 {
                //if an element >= factor is found, calculate number of successful pairs
                n = potions.len() as i32 - n;
            } else {
                //otherwise the number of successful pairs will be 0
                n = 0;
            }
            pairs.push(n);
        } 
        pairs
    }
}
