/*
Question:
A message containing letters from A-Z can be encoded into numbers using the following mapping:
'A' -> "1"
'B' -> "2"
...
'Z' -> "26"

To decode an encoded message, all the digits must be grouped then mapped back into letters
using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
    "AAJF" with the grouping (1 1 10 6)
    "KJF" with the grouping (11 10 6)

Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
Given a string s containing only digits, return the number of ways to decode it.
The test cases are generated so that the answer fits in a 32-bit integer.
*/

// Approach:
// 1) Create a fibonacci function that returns the nth fibonacci number
// 2) If the string starts with 0, return 0
// 3) Make a streak variable and a decodings variable. Make a variable to store the last character
//    and initialize it to the first character of the string
// 4) Iterate over the string from the second character
// 5) Check if the last and current character form a valid two digit number. If yes, increment the
//    streak. If they form an invalid two digit number, return 0.
//    Otherwise, if the current character is 0, multiply the decodings by fibonacci(streak+1) or by
//    fibonacci(streak+2) otherwise. Then empty the streak and set the last character to the current.
// 6) Outside the loop, end the last streak
// 7) Return the decodings

fn fibonacci(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        return b;
    }
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.starts_with("0") {
            return 0;
        }
        let mut decodings = 1;
        let mut streak = 0;
        let mut s_iter = s.chars();
        let mut last = s_iter.next().unwrap();
        for ch in s_iter {
            if (last == '1' && ch != '0')
                || (last == '2' && ch != '0' && ch != '7' && ch != '8' && ch != '9')
            {
                //true
                streak += 1;
            } else if last != '1' && last != '2' && ch == '0' {
                return 0;
            } else if streak > 0 && ch == '0' && (last == '1' || last == '2') {
                decodings *= fibonacci(streak + 1);
                streak = 0;
            } else {
                decodings *= fibonacci(streak + 2);
                streak = 0;
            }
            last = ch;
        }
        decodings *= fibonacci(streak + 2);
        decodings
    }
}
