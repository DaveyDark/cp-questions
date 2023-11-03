/*
Question:
You are given an integer array target and an integer n.

You have an empty stack with the two following operations:
    "Push": pushes an integer to the top of the stack.
    "Pop": removes the integer on the top of the stack.

You also have a stream of the integers in the range [1, n].

Use the two stack operations to make the numbers in the stack (from the bottom to the top) equal to target. You should follow the following rules:
    If the stream of the integers is not empty, pick the next integer from the stream and push it to the top of the stack.
    If the stack is not empty, pop the integer at the top of the stack.
    If, at any moment, the elements in the stack (from the bottom to the top) are equal to target, do not read new integers from the stream and do not do more operations on the stack.

Return the stack operations needed to build target following the mentioned rules. If there are multiple valid answers, return any of them.
*/

// Approach:
// 1) Make a ops vector to store the operations we have to do and a counter i to track the current
//    index of the target array we are on
// 2) Loop through numbers in the given range of 1 to n and add a push operation each time. If the
//    current number is not in the target vector, also do a pop operation, or increment i
//    if not. Then break out if the taarget vector is complete
// 3) Return the ops vector

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ops = vec!();
        let mut i = 0;
        for x in 1..=n {
            ops.push(String::from("Push"));
            if(target.len() > i && x != target[i]) { 
                ops.push(String::from("Pop"));
            }
            else { i += 1 }
            if(i == target.len() && x == target[i-1]) {break}
        }
        ops
    }
}
