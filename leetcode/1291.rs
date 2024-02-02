/*
Question:
An integer has sequential digits if and only if each digit in the number is one more than the previous digit.

Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
*/

// Approach:
// 1) Find the length of the low and high numbers
// 2) Iterate through the length of the numbers
//     For each length, iterate through the numbers from 0 to 9 - length
//     Create the number from the start index to the end + l index
//     If the number is within the range, add it to the result
//     Otherwise, break the loop
// 3) Return the result

impl Solution {
    fn make_int(s: usize, e: usize) -> i32 {
        let mut n = 0;
        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in s..e {
            n = (n * 10) + nums[i];
        }
        n
    }
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut res = vec![];
        let min_l = format!("{}", low).len();
        let max_l = format!("{}", high).len();
        for l in min_l..=max_l.min(9) {
            for i in 0..=9 - l {
                let n = Self::make_int(i as usize, (i + l) as usize);
                if n >= low && n <= high {
                    res.push(n);
                } else if n > high {
                    break;
                }
            }
        }
        res
    }
}
