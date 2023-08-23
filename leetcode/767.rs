/*
Question:
Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.

Return any possible rearrangement of s or return "" if not possible.
*/

// Approach:
// 1) Make a hashmap and then iterate over the chars in the string and record their frequency
// 2) Return empty string if one of the elements has frequency > (n+1)/2
// 3) Convert the hashmap into a vector(chars) and sort it by frequency of character
// 4) Loop until chars has 2 or more elements
//      Each iteration, push the most frequent and second most frequent character to st and reduce
//      their frequency. If their frequenct reaches 0, remove them from the vector
//      Sort the vector again at the end of iteration
// 5) Push the last remaining character(if any) in the vector and add it to st
// 6) return the string st

use std::collections::HashMap;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut map = HashMap::new();
        let mut st = String::new();
        for ch in s.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }
        if *map.values().max().unwrap() > (s.len() as i32 +1)/2 {return st}
        let mut chars: Vec<(char,i32)> = map.into_iter().collect();
        chars.sort_unstable_by_key(|x| x.1);
        while chars.len() > 1 {
            let l = chars.len();
            st.push(chars[l-1].0);
            st.push(chars[l-2].0);
            chars[l-1].1 -= 1;
            chars[l-2].1 -= 1;
            if chars[chars.len()-2].1 == 0 { chars.remove(chars.len()-2); }
            if chars[chars.len()-1].1 == 0 { chars.pop(); }
            chars.sort_unstable_by_key(|x| x.1);
        }
        if chars.len() > 0 {
            st.push(chars[0].0);
        }
        st
    }
}
