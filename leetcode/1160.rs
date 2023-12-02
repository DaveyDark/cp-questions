/*
Question:
You are given an array of strings words and a string chars.

A string is good if it can be formed by characters from chars (each character can only be used once).

Return the sum of lengths of all good strings in words.
*/

// Approach:
// 1) Make a frequency array and count the frequencies of all chars in the chars string
// 2) Loop over each word in words and count it's frequency
//      Then if all the frequencies of the word are <= the chars frequencies, we add the word
//      length to the sum
// 3) Return the sum

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut freq = vec![0; 26];
        let mut sum = 0;
        chars.bytes().for_each(|ch| freq[(ch - b'a') as usize] += 1);
        for word in words {
            let mut word_freq = vec![0; 26];
            word.bytes().for_each(|ch| word_freq[(ch - b'a') as usize] += 1);
            if (0..26).all(|i| word_freq[i] <= freq[i]) {
                sum += word.len() as i32;
            }
        }
        sum
    }
}
