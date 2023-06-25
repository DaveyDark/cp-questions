/*
Question:
You are given a string s, which contains stars *.
In one operation, you can:
    Choose a star in s.
    Remove the closest non-star character to its left, as well as remove the star itself.
Return the string after all stars have been removed.
Note:
    The input will be generated such that the operation is always possible.
    It can be shown that the resulting string will always be unique.

*/

// Appraoch
// 1) We use a stack(Vec<char>) to keep track of the solution string
// 2) We go over the given string and each iteration match the current char
//    If it's a star then we pop an element from the stack
//    Otherwise we push the current char to the stack
// 3) At the end we return the Vec<char> as a string

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '*' => { stack.pop(); }
                _ => stack.push(ch),
            }
        }
        stack.iter().collect()
    }
}
