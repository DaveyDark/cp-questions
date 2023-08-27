/*
Question:
Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.

For example:
A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28 
...
*/

// Approach:
// 1) Make variables to track the power of the current digit(power) and to store the sum(num)
// 2) Loop through the chars in given string in reverse order
// 3) Each iteration, add the value of the char multiplid by 26 to the power of power to num
// 4) Return the final value of num

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut power = 0;
        let mut num = 0;
        for ch in column_title.chars().rev() {
            num += ((ch as u8 - 'A' as u8) as i32 + 1) * 26_i32.pow(power);
            power += 1;
        }
        num
    }
}
