/*
Question:
Given an array of strings words and a character separator, split each string in words by separator.

Return an array of strings containing the new strings formed after the splits, excluding empty strings.

Notes
    separator is used to determine where the split should occur, but it is not included as part of the resulting strings.
    A split may result in more than two strings.
    The resulting strings must maintain the same order as they were initially given.
*/

// Approach:
// Fold the given words array by using a vector and extending it with an iterator over the splits
// of the word split by the given separator, filtering out any empty strings, and return it

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words
        .into_iter()
        .fold(
            Vec::new(),
            |mut vec,word| {
                vec.extend(word.split(separator)
                .filter(|wd| *wd != "")
                .map(|wd| wd.to_string()));
                vec
            }
        )
    }
}
