/*
Question:

You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.
Evaluate the expression. Return an integer that represents the value of the expression.

Note that:
    The valid operators are '+', '-', '*', and '/'.
    Each operand may be an integer or another expression.
    The division between two integers always truncates toward zero.
    There will not be any division by zero.
    The input represents a valid arithmetic expression in a reverse polish notation.
    The answer and all the intermediate calculations can be represented in a 32-bit integer.
*/

// Approach:
// 1) Use a stack to store the numbers
// 2) Iterate over the tokens
//      Try to parse the token as a number
//        If successful, push it onto the stack
//      Otherwise, pop two operands from the stack
//        Then match the operator and push the result onto the stack
// 3) Return the last element of the stack
//

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for s in tokens {
            if let Ok(n) = s.parse() {
                stack.push(n);
            } else {
                let (n2, n1) = (stack.pop().unwrap(), stack.pop().unwrap());
                stack.push(match s.as_str() {
                    "+" => n1 + n2,
                    "-" => n1 - n2,
                    "*" => n1 * n2,
                    "/" => n1 / n2,
                    _ => panic!(),
                });
            }
        }
        stack.pop().unwrap()
    }
}
