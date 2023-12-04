/*
Question:
You are given a string num representing a large integer. An integer is good if it meets the following conditions:
    It is a substring of num with length 3.
    It consists of only one unique digit.

Return the maximum good integer as a string or an empty string "" if no such integer exists.

Note:
    A substring is a contiguous sequence of characters within a string.
    There may be leading zeroes in num or a good integer.
*/

// Approach:
// 1) Loop over windows of 3 chars in the given string
// 2) Filter the windows to only those windows that contain the same 3 chars and then map them to
//    the equivalent string
// 3) Fold the iterator using "" as the default value and then find the greatest string using the
//    accumulator

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        num.into_bytes()
        .windows(3)
        .filter(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| (w[0] as char).to_string().repeat(3))
        .fold(String::new(), |acc,w| if w > acc {w} else {acc})
    }
}
