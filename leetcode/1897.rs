/*
Question:
You are given an array of strings words (0-indexed).

In one operation, pick two distinct indices i and j, where words[i] is a non-empty string, and move any character from words[i] to any position in words[j].

Return true if you can make every string in words equal using any number of operations, and false otherwise.

*/

// Approach:
// 1) Make a frequency vector to store the frequency of each character
// 2) Iterate over each word in words and store the frequency of it's chars in freq
// 3) If all frequencies are divisible by words.len(), the words can be made equal

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut freq = vec!(0; 26);
        words.iter().for_each(|word| word.bytes().for_each(|ch| freq[(ch - b'a') as usize] += 1));
        freq.iter().all(|f| f%words.len()==0)
    }
}
