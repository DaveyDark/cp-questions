/*
Question:
You are given an integer n, the number of teams in a tournament that has strange rules:
    If the current number of teams is even, each team gets paired with another team. 
    A total of n / 2 matches are played, and n / 2 teams advance to the next round.
    If the current number of teams is odd, one team randomly advances in the tournament, 
    and the rest gets paired. A total of (n - 1) / 2 matches are played, and (n - 1) / 2 + 1 teams advance to the next round.

Return the number of matches played in the tournament until a winner is decided.
*/

// Approach:
// 1) Make a counter
// 2) Loop until n becomes 1
//      Each iteration add n/2 to matches
//      Then update n to n/2 if it's even or n/2 + 1 if it's odd
// 3) Return matches

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut matches = 0;
        while n != 1 {
            matches += n/2;
            n = if n%2==0 {n/2} else {n/2 + 1};
        }
        matches
    }
}
