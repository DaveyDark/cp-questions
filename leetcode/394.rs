/*
Question:
Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], 
where the encoded_string inside the square brackets is being repeated exactly k times. 
Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, 
square brackets are well-formed, etc. Furthermore, 
you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. 
For example, there will not be input like 3a or 2[4].

The test cases are generated so that the length of the output will never exceed 105.
*/

// Approach:
// Create a decode() function that can recursively decode each layer of encoding and call it on the
// base string with a repeat of 1 to decode the string.
// 1) Make variables to store the decoded string, the (inner) encoded string, the current number of
//    brackets, weather the current inner string is encoded, and the repeat amount for the inner
//    encoded string
// 2) Loop over the cars in the given string
//      If the current slice is encoded then we push the current char to the encoded string
//
//      If the brackets are 0 and we encounter a digit, then it is the start of an encoded string
//      and the digits are the repeat number, so we store it in r
//      Else if we encounter a [, we increase the brackets counter or decrease it if we encounter a ]
//      Lastly if the current slice is not encoded we directly push the char into the decoded
//      string
//
//      Then if the brackets are 0 and the slice was encoded, the encoded string has ended so we
//      pass it recursively to decode() to decode it and add that to the decoded string. Then we
//      also reset all the encoded string related variables
// 3) Then we repeat the decoded string the given number of times and return it
//

impl Solution {
    pub fn decode_string(s: String) -> String {
        Self::decode(&s, 1)
    }
    fn decode(s: &String, repeat: i32) -> String {
        let mut decoded_str = String::new();
        let mut encoded_str = String::new();
        let mut brackets = 0;
        let mut encoded = false;
        let mut r = 0;
        for ch in s.chars() {
            if encoded {
                encoded_str.push(ch);
            }
            if brackets == 0 && ch.is_ascii_digit() {
                r = (r*10) + ch.to_digit(10).unwrap() as i32;
            } else if ch == '[' {
                encoded = true;
                brackets += 1;
            } else if ch == ']' {
                brackets -= 1;
            } else if !encoded {
                decoded_str.push(ch);
            }
            if encoded && brackets == 0 {
                decoded_str += Self::decode(&encoded_str, r).as_str();
                r = 0;
                encoded = false;
                encoded_str.clear();
            }
        }
        decoded_str.repeat(repeat as usize)
    }
}
