/*
A string s is called good if there are no two different characters in s that have the same frequency.
Given a string s, return the minimum number of characters you need to delete to make s good.
The frequency of a character in a string is the number of times it appears in the string. 
For example, in the string "aab", the frequency of 'a' is 2, while the frequency of 'b' is 1.
*/

// Approach:
// 1) Create a counter to store the nubmer of operations and a vector to store the frequency of
//    letters in the string
// 2) Loop over chars in the string and update the respective frequency in freq
// 3) Sort the frequencies
// 4) Loop over the frequencies in reverse order
//      If the current freq is more than the last one, we decrease it until it isn't and increase
//      ops
// 5) Return ops

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut ops = 0;
        let mut freq = vec!(0; 26);
        s.chars().for_each(|ch| freq[(ch as u8 - 'a' as u8) as usize] += 1);
        freq.sort_unstable();
        for i in (0..freq.len()-1).rev() {
            while freq[i] >= freq[i+1] && freq[i] > 0 {
                freq[i] -= 1;
                ops += 1;
            }
        }
        ops
    }
}
