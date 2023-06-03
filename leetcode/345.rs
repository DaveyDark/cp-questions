/*
Question:
Given a string s, reverse only all the vowels in the string and return it.
The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.
*/

// Approach:
// 1) We make a mutable vector out of the given string
// 2) We iterate over the vector and store all vowels in a vector as we come across them
// 3) Then we again iterate over the vector and when we enounter any vowel we replace it with the character popped from the vowels vector
// 4) Then we collect the vector into a String and return it

impl Solution {
    fn is_vowel(ch: char) -> bool {
        match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = Vec::new();
        let mut st: Vec<char> = s.chars().collect();

        for ch in st.iter() {
            if Self::is_vowel(*ch) { 
                vowels.push(*ch) 
            }
        }

        for ch in st.iter_mut() {
            if Self::is_vowel(*ch) {
                *ch = vowels.pop().unwrap();
            } 
        }

        st.into_iter().collect()
    }
}
