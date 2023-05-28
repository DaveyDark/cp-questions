/*
Question:
You are keeping the scores for a baseball game with strange rules. At the beginning of the game, you start with an empty record.
You are given a list of strings operations, where operations[i] is the ith operation you must apply to the record and is one of the following:
    An integer x:
        Record a new score of x.
    '+':
        Record a new score that is the sum of the previous two scores.
    'D':
        Record a new score that is the double of the previous score.
    'C':
        Invalidate the previous score, removing it from the record.
Return the sum of all the scores on the record after applying all the operations.
The test cases are generated such that the answer and all intermediate calculations fit in a 32-bit integer and that all operations are valid.
*/

// Approach:
// 1) We make a vector for keeping record of the points
// 2) We go through all given operations and match them with the given cases
//    For "+" we push the sum of last 2 elements to the record
//    For "D" we push the double of last element to the record
//    For "C" we pop the last element of the record
//    Otherwise we pusht he given number to the record directly
// 3) At the end we return the sum of the record vector

impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut record = Vec::new();
        for op in operations.iter() {
            match op.as_str() {
                "+" => { record.push(record[record.len()-1] + record[record.len()-2] ) }
                "D" => { record.push(*record.last().unwrap() * 2) }
                "C" => { record.pop(); }
                _ => { record.push(op.parse::<i32>().unwrap()) }
            }
        }
        record.iter().sum()
    }
}
