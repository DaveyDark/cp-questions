/*
You are climbing a staircase. It takes n steps to reach the top.
Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
*/

// Approach:
// The answer is just the nth value in the fibonacci series 
// So we just run a loop to calculate fibonacci series upto n and return the value

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        for i in 0..n {
            let tmp = b;
            b = a+b;
            a = tmp;
        }
        b
    }
}
