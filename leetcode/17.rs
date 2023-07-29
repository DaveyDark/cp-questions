/*
Question:
Given a string containing digits from 2-9 inclusive, 
return all possible letter combinations that the number could represent. 
Return the answer in any order.
A mapping of digits to letters (just like on the telephone buttons) is given below. 
Note that 1 does not map to any letters.
*/

// Approach:
// 1) Define map_digit that will return the Vec<String> corresponding to the given numpad key
// 2) If digits is empty, we return an empty string
// 3) If digits has only 1 digit, we return the corresponding Vec<String> using map_digit
// 4) Otherwise, we calculate the letter combinations for the last digit(a) and the other digits except the last(b)
//    Then we do a nested loop to go over the combinations of chars in a and b, and push each combinations into the list
//    After that we return the generated list

use std::collections::HashMap;

impl Solution {
    fn map_digit(digit: char) -> Vec<String> {
        let mut list = match digit {
            '2' => vec!("a","b","c"),
            '3' => vec!("d","e","f"),
            '4' => vec!("g","h","i"),
            '5' => vec!("j","k","l"),
            '6' => vec!("m","n","o"),
            '7' => vec!("p","q","r","s"),
            '8' => vec!("t","u","v"),
            '9' => vec!("w","x","y","z"),
            _ => vec!(),
        };
        list.iter().map(|x| x.to_string()).collect()
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec!()
        }
        if digits.len() == 1 {
            return Self::map_digit(digits.chars().next().unwrap());
        }
        let mut list = Vec::new();
        let mut a = Self::letter_combinations(digits[digits.len()-1..].to_string());
        let mut b = Self::letter_combinations(digits[..digits.len()-1].to_string());
        for st1 in a.iter() {
            for st2 in b.iter() {
                list.push(st2.to_string() + st1);
            }
        }
        list
    }
}
