/*
Question:
You are given an array of strings arr.
A string s is formed by the concatenation of a subsequence of arr that has unique characters.

Return the maximum possible length of s.

A subsequence is an array that can be derived from another array by deleting some
or no elements without changing the order of the remaining elements.
*/

// Approach:
// 1) For each string, count the frequency of each character.
// 2) Iterate over the strings and for each string, check if the frequency of each character is < 1
//    and skip the word if it isn't
//    otherwise, call the combine function with the current string and the current frequency vector.
//    In combine(), iterate over the remaining strings and for each string, check if the frequency of
//    the sum of the current frequency vector and the frequency vector of the current string is < 1.
//    If it is, call combine() with the current frequency vector and the current string.
// 3) Return the maximum length returned by the combine() function.

impl Solution {
    fn combine(arr: &Vec<String>, freq: &Vec<Vec<i32>>, f: Vec<i32>, i: usize, len: usize) -> i32 {
        let mut max = len as i32;
        for j in i..arr.len() {
            let mut sum: Vec<i32> = f.iter().enumerate().map(|(i, &x)| x + freq[j][i]).collect();
            if sum.iter().all(|&x| x <= 1) {
                max = max.max(Self::combine(arr, freq, sum, j + 1, len + arr[j].len()));
            }
        }
        max
    }
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut freq = vec![vec![0; 26]; arr.len()];
        for i in 0..arr.len() {
            for ch in arr[i].bytes() {
                freq[i][(ch - b'a') as usize] += 1;
            }
        }

        let mut len = 0;
        for i in 0..freq.len() {
            if freq[i].iter().any(|&x| x > 1) {
                continue;
            }
            len = len.max(Self::combine(&arr, &freq, freq[i].clone(), 0, arr[i].len()));
        }
        len
    }
}
