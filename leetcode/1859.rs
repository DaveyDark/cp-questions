/*
Question:
A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of lowercase and uppercase English letters.
A sentence can be shuffled by appending the 1-indexed word position to each word then rearranging the words in the sentence.
    For example, the sentence "This is a sentence" can be shuffled as "sentence4 a3 is2 This1" or "is2 sentence4 This1 a3".
Given a shuffled sentence s containing no more than 9 words, reconstruct and return the original sentence.
*/

// Approach:
// 1) We first go over each word in the sentence and add it to a hashmap, sperating the integer index as key and the rest of the word as value
// 2) Then we loop from 1 to 9, which is the given range of the index in the question constraints
//    For each iteratioon we add the value at that index if it exists otherwise just an emptry string, followed by a space
// 3) We return the string trimmed of the extra whitespace at the end

use std::collections::HashMap;

impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut map: HashMap<i32,String> = HashMap::new();
        let mut ans = String::new();
        for word in s.split(' ') {
            map.insert(word[word.len()-1..].parse().unwrap(),word[..word.len()-1].to_string());
        }
        for i in 1..10 {
            ans.push_str(map.get(&i).unwrap_or(&String::new()));
            ans.push(' ');
        }
        ans.trim().to_string()
    }
}
