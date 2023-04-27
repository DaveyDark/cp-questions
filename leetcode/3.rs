/*
Question:
Given a string s, find the length of the longest
substring
without repeating characters.
*/

// Approach:
// 1) We make a buffer which is a vector of chars to simulate a sliding window by storing the elements inside the window
// 2) We go over the string and check if the current char already exists in the buffer
//    If it does then we start removing elements from the front of the vector until it is removed
// 3) Then we push the current char into the buffer
// 4) At the end of the iteration we check if the current window length is more than the longest recorded
//    If it is then we set it to the current window length
// 5) We return the longest window length at the end

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest: i32 = 0;
        let mut buffer = Vec::new();
        for (i,ch) in s.char_indices() {
            while buffer.contains(&ch) {
                buffer.remove(0);
            }
            buffer.push(ch);
            if buffer.len() > longest as usize {longest = buffer.len() as i32;}
        }
        longest
    }
}
