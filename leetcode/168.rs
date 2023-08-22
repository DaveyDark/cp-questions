/*
Question:
Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
*/

// Approach:
// Basically we have to convert a base 10 number to a base 26 number, offset by 1 because instead
// of 0-Y, we need to output in A-Z 
// 1) Create a chars vector which will store the resulting string
// 2) Loop while column number > 0
// 3) Decrement the column number to account for the +1 offset in the number system
// 4) Then map the mod of column_number with 26 to a letter and push it to the string and then divide column_number by 26
// 5) Outside the loop, Reverse the vector and return it as a string

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut st = Vec::new();
        while column_number > 0 {
            column_number -= 1;
            st.push(('A' as u8 + (column_number%26) as u8) as char);
            column_number /= 26;
        }
        st.iter().rev().collect()
    }
}
