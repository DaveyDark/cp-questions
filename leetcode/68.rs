/*
Question:
Given an array of strings words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.

You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.

Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.

For the last line of text, it should be left-justified, and no extra space is inserted between words.

Note:
    A word is defined as a character sequence consisting of non-space characters only.
    Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
    The input array words contains at least one word.
*/

// Approach:
// 1) Make a vector to store the final text
// 2) Loop until the words vector is empty
// 3) Each iteration, we gather the words to put in the current line
//      To do this, keep adding words to a vector while their combined length plus the spaces
//      between them is less than the max_width
// 4) After the line is formed, justify it using left_justify() if it's the last line or if the line 
//      only has one word, otherwise justify it using justify_line(), and push the result to text
//
// To justify a line, we first need to calculate the spaces between words. Start by diving the
// total space evenly to the nearest integer. Then add the remainder space to the starting spaces.
// Then we go over all the words for the given line, add it to the string and then also add the
// respective space for that word
// 
// to justify a line to the lift, we first simply join all words with a single space and then fill
// all the reamining space with whitespace at the end

impl Solution {
    fn justify_line(mut line: Vec<String>, max_width: i32) -> String {
        let mut st = String::new();
        let space = max_width - line.iter().fold(0,|acc,x| acc + x.len()) as i32;
        let mut gaps = vec!((space/(line.len()-1) as i32); line.len()-1);
        for i in 0..space%(line.len()-1) as i32 {
            gaps[i as usize] += 1;
        }
        for i in 0..line.len() {
            st += &line[i];
            st += &" ".repeat(*gaps.get(i).unwrap_or(&0) as usize);
        }
        st
    }
    fn left_justify(mut line: Vec<String>, max_width: i32) -> String {
        let mut st = line.join(" ");
        st += &" ".repeat(max_width as usize - st.len());
        st
    }
    pub fn full_justify(mut words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut text = Vec::new();
        while !words.is_empty() {
            let mut line = Vec::new();
            let mut sum = 0;
            while !words.is_empty() && sum + 1 + words[0].len() as i32 <= max_width + 1 {
                sum += words[0].len() as i32 + 1;
                line.push(words.remove(0));
            }
            if line.len() == 1 || words.is_empty() {
                text.push(Self::left_justify(line, max_width));
            } else {
                text.push(Self::justify_line(line, max_width));
            }
        }
        text
    }
}
