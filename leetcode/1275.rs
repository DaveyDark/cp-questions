/*
Question:
Tic-tac-toe is played by two players A and B on a 3 x 3 grid. The rules of Tic-Tac-Toe are:
    Players take turns placing characters into empty squares ' '.
    The first player A always places 'X' characters, while the second player B always places 'O' characters.
    'X' and 'O' characters are always placed into empty squares, never on filled ones.
    The game ends when there are three of the same (non-empty) character filling any row, column, or diagonal.
    The game also ends if all squares are non-empty.
    No more moves can be played if the game is over.
Given a 2D integer array moves where moves[i] = [rowi, coli] indicates that the ith move will be played on grid[rowi][coli]. 
return the winner of the game if it exists (A or B). In case the game ends in a draw return "Draw". If there are still movements to play return "Pending".
You can assume that moves is valid (i.e., it follows the rules of Tic-Tac-Toe), the grid is initially empty, and A will play first.
*/

// Approach:
// 1) We make a 2d vector to store the current state of the board
// 2) Then we go over the given moves one by one and check for a win using the check_win method
//    check_win checks if any of the winning combinations are present on the board
//    if any winner is found then it is returned
// 3) At the end of the loop, we check if the number is moves is 9
//    If it is then we return a draw
//    Otherwise we return pending since the board is not fully filled

impl Solution {
    pub fn check_win(board: &Vec<Vec<usize>>) -> bool {
        let winning_combinations = [
            // Rows
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            // Columns
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            // Diagonals
            (0, 4, 8),
            (2, 4, 6),
        ];

        for &(a, b, c) in winning_combinations.iter() {
            if board[a / 3][a % 3] != 0 &&
            board[a / 3][a % 3] == board[b / 3][b % 3] &&
            board[a / 3][a % 3] == board[c / 3][c % 3] {
                return true;
            }
        }
        false
    }
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec!(vec!(0,0,0),vec!(0,0,0),vec!(0,0,0));
        for i in 0..moves.len() {
            board[moves[i][0] as usize][moves[i][1] as usize] = 1 + (i%2);
            if Self::check_win(&board) {
                return if i%2==0 {String::from("A")} else {String::from("B")};
            }
        }
        if moves.len() == 9 {String::from("Draw")} else {String::from("Pending")}
    }
}
