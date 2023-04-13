/*
Question:
Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
    Each row must contain the digits 1-9 without repetition.
    Each column must contain the digits 1-9 without repetition.
    Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
Note:
    A Sudoku board (partially filled) could be valid but is not necessarily solvable.
    Only the filled cells need to be validated according to the mentioned rules.
*/

// Approach:
// 1) We make a hashmap that we will use for checking every row/column/box
// 2) We first check rows by simple looping through each element in each row in the array
//    If the element is '.' then we skip adding it to the map
//    Then we calculatr frequency in the map, and then chack if any number has frequency > 1
//    and then return false if it does
// 3) Then we check the columns by looping a variable through 0-8 and using it as index for each row in the matrix
//    and then following the same procedure as above to validate it
// 4) For boxes, we use 2 loops through 0-2 to go through the boxes and another 2 loops from 0-2 to go through each individual box
//    Frequency is calculated and validated as described above
// 5) If we reach the end of the function then we return true

use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut map = HashMap::new();
        //check rows
        for row in &board {
            for cell in row {
                if *cell == '.' {continue;}
                let val = map.entry(*cell).or_insert(0);
                *val += 1;
            }
            for (key,val) in &map {
                if *val > 1 {
                    return false;
                }
            }
            map = HashMap::new();
        }
        //check columns
        for i in 0..9 {
            for row in &board {
                if row[i] == '.' {continue;}
                let val = map.entry(row[i]).or_insert(0);
                *val += 1;
            }
            for (key,val) in &map {
                if *val > 1 {
                    return false;
                }
            }
            map = HashMap::new();
        }
        //check boxes
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        if board[(3*i)+k][(3*j)+l] == '.' {continue;}
                        let val = map.entry(board[(3*i)+k][(3*j)+l]).or_insert(0);
                        *val += 1;
                    }
                }
                for (key,val) in &map {
                    if *val > 1 {
                        return false;
                    }
                }
                map = HashMap::new();
            }
        }
        true
    }
}
