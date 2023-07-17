/*
Question:
In the world of Dota2, there are two parties: the Radiant and the Dire.

The Dota2 senate consists of senators coming from two parties. 
Now the Senate wants to decide on a change in the Dota2 game. 
The voting for this change is a round-based procedure. 
In each round, each senator can exercise one of the two rights:
    Ban one senator's right: A senator can make another senator lose all his rights in this and all the following rounds.
    Announce the victory: If this senator found the senators who still have rights to vote are all from the same party, 
                          he can announce the victory and decide on the change in the game.

Given a string senate representing each senator's party belonging. 
The character 'R' and 'D' represent the Radiant party and the Dire party. 
Then if there are n senators, the size of the given string will be n.
The round-based procedure starts from the first senator to the last senator in the given order. 
This procedure will last until the end of voting. All the senators who have lost their rights will be skipped during the procedure.
Suppose every senator is smart enough and will play the best strategy for his own party.
Predict which party will finally announce the victory and change the Dota2 game. The output should be "Radiant" or "Dire".
*/

// Approach:
// We can use a stack and queue to simulate the voting
// The stack represents the current upopposed vote list
// The Queue represents the rest of the unprocessed senate
// 1) Make a stack and queue
// 2) While the queue isn't empty, pop the front of the queue and process it
//    If the stack is empty or has members of the same party, move the current vote to the stack
//    Otherwise if the stack has votes of the opposite party, pop the stack and add the popped element back to queue
//      This is because the vote is the stack of the opposing party occurs before the current vote
//      So if there is opposition as in this case, that senator would instead use his vote to void the current senator
//      This is what is simulated in this situation, the current sentor vote is discarded and the opposing sentor moves back in the queue to move in another round
// 3) After the queue is processed, the stack would be containing votes of the winning party
//    So just take an element from the stack and check the party and return the respective String

use std::collections::VecDeque;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut stack: Vec<char> = vec![];
        let mut queue: VecDeque<char> = senate.chars().collect();

        while !queue.is_empty() {
            let top = queue.pop_front().unwrap();

            if stack.is_empty() || *stack.last().unwrap() == top {
                stack.push(top);
            } else {
                queue.push_back(stack.pop().unwrap());
            }
        }

        if stack.pop().unwrap() == 'R' { "Radiant".to_string() } else { "Dire".to_string() }
    }
}
