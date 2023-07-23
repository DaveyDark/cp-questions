/*
Question:
Two strings are considered close if you can attain one from the other using the following operations:

    Operation 1: Swap any two existing characters.
        For example, abcde -> aecdb
    Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
        For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)

You can use the operations on either string as many times as necessary.

Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
*/

// Approach:
// From the operations, we can derive that two strings are close if the following conditions are satisfied
//    They have the same letters
//    Their frequencies, regardless of what letter they are for, are the same numbers
// 1) If the lengths of the strings are unequal, we can just return false
// 2) We make 2 vectors to store teh frequencies of letters and then populate them with the frequencies of letters from both strings
// 3) Then we go through both the vectors and if there's a letter than exists in one string but not the other, we return false
// 4) Then we sort the two vectors, and if they are equal return true, otherwise return false

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {return false}
        let mut freq1 = vec!(0; 26);
        let mut freq2 = vec!(0; 26);
        for ch in word1.chars() {freq1[ch as usize - 97] += 1}
        for ch in word2.chars() {freq2[ch as usize - 97] += 1}
        for i in 0..25 {
            if (freq1[i] != 0 && freq2[i] == 0) || (freq1[i] == 0 && freq2[i] != 0) { return false }
        }
        freq1.sort();
        freq2.sort();
        freq1 == freq2
    }
}
