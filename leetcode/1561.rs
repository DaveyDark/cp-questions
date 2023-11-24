/*
Question:
There are 3n piles of coins of varying size, you and your friends will take piles of coins as follows:
    In each step, you will choose any 3 piles of coins (not necessarily consecutive).
    Of your choice, Alice will pick the pile with the maximum number of coins.
    You will pick the next pile with the maximum number of coins.
    Your friend Bob will pick the last pile.
    Repeat until there are no more piles of coins.

Given an array of integers piles where piles[i] is the number of coins in the ith pile.

Return the maximum number of coins that you can have.
*/

// Appraoch:
// For the most optimal picking, we will pick the 2 greatest piles and the lowest pile, from which
// Alice gets the greatest and Bob gets the lowest pile and so we get the 2nd greatest pile. To
// maximize the coins we get in our pile we can sort the piles and pick the rightmost 2 and the
// leftmost pile everytime and add the coins in the 2nd greatest pile.
// 1) Sort the piles
// 2) Iterate over the piles and skip the first n/3 piles(they will be Bob's), then step by 2 for
//    the remaining piles(the second pile each time is Alice's so we step 2) and then sum these
//    elements and return them

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_unstable();
        piles.iter().skip(piles.len()/3).step_by(2).fold(0, |acc, p| acc+p)
    }
}
