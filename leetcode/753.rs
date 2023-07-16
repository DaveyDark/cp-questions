/*
Question:
We are given an array asteroids of integers representing asteroids in a row.
For each asteroid, the absolute value represents its size, and the sign represents its direction 
(positive meaning right, negative meaning left). Each asteroid moves at the same speed.
Find out the state of the asteroids after all collisions. 
If two asteroids meet, the smaller one will explode. 
If both are the same size, both will explode. 
Two asteroids moving in the same direction will never meet.
*/

// Approach:
// We use a stack and pile to keep track of the asteroids
// The stack moves through the given vector from left to right and stores any colliding or positive asteroids
// The pile collects the left moving(negative) asteroids that pass out from the stack
// At the end we can combine the pile and stack and return the vector as the solution
// 1) Make a stack and pile vector
// 2) Go through the asteroids vector
//    If the asteroid is positive we push it to the stack
//    If it's negative but the stack is empty, it will just pass through to the pile, so we push it to the pile directly
//    If it's negative but the stack isn't empty, then we take the front of the stack
//      While the stack is not empty and the front is less than the attacking asteroid, we keep popping the stack
//      If the last element we checked is less than the asteroid still, it means the stack emptied so the asteroid passes through to the pile
//      So we push the asteroid to the pile
//      Otherwise it means the asteroid gets destroyed so we need to do nothing in that case
// 3) We add the stack to the back of pile to create the final asteroid sequence
// 4) We return the appended pile

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut pile = Vec::new();
        for &ast in asteroids.iter() {
            if ast > 0 {
                stack.push(ast);
            } else if !stack.is_empty() {
                let mut x = *stack.last().unwrap();
                while !stack.is_empty() {
                    x = *stack.last().unwrap();
                    if x>ast.abs() {break}
                    stack.pop();
                    if x==ast.abs() {break}
                }
                if x < ast.abs() {
                    pile.push(ast);
                }
            } else {
                pile.push(ast);
            }
        }
        pile.append(&mut stack);
        pile
    }
}
