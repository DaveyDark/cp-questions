/*
Question:
Given an array of integers temperatures represents the daily temperatures,
return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature.
If there is no future day for which this is possible, keep answer[i] == 0 instead.
*/

// Approach:
// 1) Create a stack of (temperature, index) pairs and a ans vector.
// 2) Loop over the temperatures from the end.
// 3) While the stack is not empty and the last temperature in the stack is less than the current
//    temp, pop the stack.
// 4) Push the difference between the top of the stack and the current index into the ans vector.
// 5) Push the current temperature and index into the stack.
// 6) Outside the loop, return the reversed ans vector.

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = vec![];
        let mut ans = vec![];
        for (i, t) in temperatures.into_iter().enumerate().rev() {
            while !stack.is_empty() && stack.last().unwrap().0 <= t {
                stack.pop();
            }
            ans.push((stack.last().unwrap_or(&(0, i)).1 - i) as i32);
            stack.push((t, i));
        }
        ans.into_iter().rev().collect()
    }
}
