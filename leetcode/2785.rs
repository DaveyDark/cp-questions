/*
Question:
Given a 0-indexed string s, permute s to get a new string t such that:
    All consonants remain in their original places. 
    More formally, if there is an index i with 0 <= i < s.length such that s[i] is a consonant, 
    then t[i] = s[i].
    The vowels must be sorted in the nondecreasing order of their ASCII values. 
    More formally, for pairs of indices i, j with 0 <= i < j < s.length such that s[i] and s[j] are vowels, 
    then t[i] must not have a higher ASCII value than t[j].

Return the resulting string.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in lowercase or uppercase. Consonants comprise all letters that are not vowels.
*/

// Appraoch:
// 1) Make a vector to store the frequencies of vowels and another array to store the mapping of
//    each index to the vowel. Since we only need to track 10 chars, we can assign each of them
//    indicies 0-9 and store the mapping in this map
// 2) Loop over the given string and update the frequency of vowels in the freq vector
// 3) Make a string to store the new string
// 4) Loop over the string again then for each char
//      If it is a vowel we find the vowel with a non zero frequency and push it to the new string
//      Otherwise in case of a consonent we directly push the current char
// 5) Return the new string

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels = vec!(0; 10);
        let vowel_map = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u',];

        for (i,ch) in s.chars().enumerate() {
            match ch.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowels[vowel_map.iter().enumerate().find(|(_,c)| **c == ch).unwrap().0] += 1;
                },
                _ => (),
            }
        }

        let mut new_str = String::new();

        for ch in s.chars() {
            match ch.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    let mut i = 0;
                    loop {
                        if vowels[i] > 0 {break}
                        i += 1;
                    }
                    vowels[i] -= 1;
                    new_str.push(vowel_map[i]);
                },
                _ => {
                    new_str.push(ch);
                },
            }
        }
        new_str
    }
}
