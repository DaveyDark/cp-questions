/*
Question:
You are given a string s consisting only of the characters '0' and '1'.
In one operation, you can change any '0' to '1' or vice versa.

The string is called alternating if no two adjacent characters are equal.
For example, the string "010" is alternating, while the string "0100" is not.

Return the minimum number of operations needed to make s alternating.
*/

// Approach:
// 1) Make a function that checks the number of changes required to make a string
// 2) Call the function with '0' and '1' as starting characters and take the minimum
// 3) Return the minimum of the two calls, or the length of the string otherwise

impl Solution {
    fn check_string(start: char, s: &String) -> i32 {
        let mut last = start.clone();
        let mut changes = 0;
        for ch in s.chars() {
            if ch == last {
                changes += 1;
                last = if last == '0' { '1' } else { '0' };
            } else {
                last = ch.clone()
            }
        }
        changes
    }
    pub fn min_operations(s: String) -> i32 {
        Self::check_string('0', &s).min(Self::check_string('1', &s).min(s.len() as i32))
    }
}
