/*
Question:
The chess knight has a unique movement, 
it may move two squares vertically and one square horizontally, 
or two squares horizontally and one square vertically (with both forming the shape of an L). 
The possible movements of chess knight are shown in this diagaram:

A chess knight can move as indicated in the chess diagram below:

We have a chess knight and a phone pad as shown below, the knight can only stand on a numeric cell (i.e. blue cell).

Given an integer n, return how many distinct phone numbers of length n we can dial.

You are allowed to place the knight on any numeric cell initially 
and then you should perform n - 1 jumps to dial a number of length n. 
All jumps should be valid knight jumps.

As the answer may be very large, return the answer modulo 109 + 7.
*/

// Approach:
// 1) Make a vector to store the current and last row of the dp array. The current array will be
//    init to 10 0s and the last array to 10 1s
// 2) Loop n-1 times and update the current row using the last row.
//      We can move to 4 and 6 from 0 so the 0th index is the sum of 4th and 6th index of last
//      similarly update the other 9 indices, applying MOD to each one as well
//      Then swap last and curr
// 3) Sum the last vector as u64 to prevent overflow and mod it then convert it to i32 and return

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut curr = vec!(0; 10);
        let mut last = vec!(1; 10);
        for _ in 1..n {
            curr[0] = (last[4] + last[6]) % MOD;
            curr[1] = (last[6] + last[8]) % MOD;
            curr[2] = (last[7] + last[9]) % MOD;
            curr[3] = (last[4] + last[8]) % MOD;
            curr[4] = (last[0] + last[3] + last[9]) % MOD;
            curr[5] = 0;
            curr[6] = (last[0] + last[1] + last[7]) % MOD;
            curr[7] = (last[2] + last[6]) % MOD;
            curr[8] = (last[1] + last[3]) % MOD;
            curr[9] = (last[2] + last[4]) % MOD;
            std::mem::swap(&mut last,&mut curr);
        }
        (last.iter().sum::<u64>() % MOD) as i32
    }
}
