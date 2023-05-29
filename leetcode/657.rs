/*
Question:
There is a robot starting at the position (0, 0), the origin, on a 2D plane. Given a sequence of its moves, judge if this robot ends up at (0, 0) after it completes its moves.
You are given a string moves that represents the move sequence of the robot where moves[i] represents its ith move. Valid moves are 'R' (right), 'L' (left), 'U' (up), and 'D' (down).
Return true if the robot returns to the origin after it finishes all of its moves, or false otherwise.
Note: The way that the robot is "facing" is irrelevant. 'R' will always make the robot move to the right once, 
'L' will always make it move left, etc. Also, assume that the magnitude of the robot's movement is the same for each move.
*/

// Approach:
// 1) We keep track of the current position in a tuple
// 2) We iterate over the characters in moves and match each char with the given commands and modify the position accordingly
// 3) At the end we check if the resulting position is the origin and return the result of that expression

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut pos = (0,0);
        for mv in moves.chars() {
            match mv {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => (),
            }
        }
        pos == (0,0)
    }
}
