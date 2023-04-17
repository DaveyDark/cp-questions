/*
Question
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
An input string is valid if:
    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.

*/

// Approach:
// 1) We make a vector to keep track of open brackets and then loop through the string
// 2) If the current char is an open bracket then we push it into the vector
// 3) Otherwise we pop the last element of the vector and see if it matches the current character
//    And we return false if it doesn't
// 4) If we reach the end of the function and brackets is empty then we return true otherwise false

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                brackets.push(ch);
            } else {
                let x = brackets.pop().unwrap_or('!');
                if !((x == '(' && ch == ')') || (x == '{' && ch == '}') || (x == '[' && ch == ']')) {
                    // not a matching bracket
                    return false;
                }
            }
        }
        brackets.len() == 0
    }
}
