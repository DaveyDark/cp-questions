/*
Question:
Along a long library corridor, there is a line of seats and decorative plants. 
You are given a 0-indexed string corridor of length n consisting of letters 'S' and 'P' 
where each 'S' represents a seat and each 'P' represents a plant.

One room divider has already been installed to the left of index 0, 
and another to the right of index n - 1. Additional room dividers can be installed. 
For each position between indices i - 1 and i (1 <= i <= n - 1), at most one divider can be installed.

Divide the corridor into non-overlapping sections, 
where each section has exactly two seats with any number of plants. 
There may be multiple ways to perform the division. 
Two ways are different if there is a position with a room divider 
installed in the first way but not in the second way.

Return the number of ways to divide the corridor. 
Since the answer may be very large, return it modulo 109 + 7. 
If there is no way, return 0.
*/

// Approach:
// 1) Create variables to store the number of seats so far, the number of ways and the current gap
//    length.
// 2) Trim the corridor string to remove leading and tailing Ps and return 0 if the trimmed string
//    is empty
// 3) Loop over chats in the string
//      If the current char is a seat, we increment seats. Also if the gap is non zero, we multiply
//      ways by gap+1 and then MOD it to prevent overflow and reset gap
//      Otherwise if the number if seats is even(i.e. we are in a gap) we increment gap
// 4) If the number of seats if odd we return 0 since it is impossible to divide in a valid  way
// 5) Return ways as a i32

const MOD: u64 = 1_000_000_007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut seats = 0;
        let mut ways =  1u64;
        let mut gap = 0;
        let corridor = corridor.trim_matches('P');
        if corridor.len() == 0 {return 0}
        for ch in corridor.chars() {
            if ch == 'S' {
                if gap > 0 {
                    ways *= (gap+1);
                    ways %= MOD;
                    gap = 0;
                }
                seats += 1;
            }
            else {
                if seats%2 == 0 {
                    gap += 1;
                }
            }
        }
        if seats%2 == 1 {return 0}
        ways as i32
    }
}
