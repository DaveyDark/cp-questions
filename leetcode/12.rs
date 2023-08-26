/*
Question:
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
    I can be placed before V (5) and X (10) to make 4 and 9. 
    X can be placed before L (50) and C (100) to make 40 and 90. 
    C can be placed before D (500) and M (1000) to make 400 and 900.
Given an integer, convert it to a roman numeral.
*/

// Approach:
// 1) Make arrays to store the roman representation of the ones, tens, hundreds and thousands place
// 2) Append the string with the index of (num/1000)%10 in the thousands array
//    This expression gets the digit in the thousands(1000) place and reduces it to a single digit
// 3) Similarly append the strings for the hundreds, tens and ones place
// 4) Return the final string

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut roman = String::new();
        let ones = ["","I","II","III","IV","V","VI","VII","VIII","IX"];
        let tens = ["","X","XX","XXX","XL","L","LX","LXX","LXXX","XC"];
        let hundreds = ["","C","CC","CCC","CD","D","DC","DCC","DCCC","CM"];
        let thousands = ["","M","MM","MMM"];
        roman += thousands[((num/1000)%10) as usize];
        roman += hundreds[((num/100)%10) as usize];
        roman += tens[((num/10)%10) as usize];
        roman += ones[(num%10) as usize];
        roman
    }
}
