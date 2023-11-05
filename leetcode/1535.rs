/*
Question:
Given an integer array arr of distinct integers and an integer k.

A game will be played between the first two elements of the array (i.e. arr[0] and arr[1]). 
In each round of the game, we compare arr[0] with arr[1], the larger integer wins and remains at position 0, 
and the smaller integer moves to the end of the array. The game ends when an integer wins k consecutive rounds.

Return the integer which will win the game.

It is guaranteed that there will be a winner of the game.
*/

// Approach:
// 1) If k >= array length then we just return the max element
// 2) Make variables to track the current element being compared, the base element to compare and
//    the current winstreak
// 3) Loop through the array
//      If the base element is more than the current then we increase wins
//      Else we reset the wins and make the current element the base
//      If the number of wins is k then we break out
// 4) Return the base element at the end

impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        if k >= arr.len() as i32 { return *arr.iter().max().unwrap_or(&0) }
        let mut wins = 0;
        let mut base = 0;
        for n in 1..arr.len() {
            if arr[base] > arr[n] {
                wins += 1;
            }
            else {
                wins = 1;
                base = n;
            }
            if wins == k {break}
        }
        arr[base]
    }
}
