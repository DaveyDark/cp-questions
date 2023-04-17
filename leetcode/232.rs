/*
Question:
Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
Implement the MyQueue class:
    void push(int x) Pushes element x to the back of the queue.
    int pop() Removes the element from the front of the queue and returns it.
    int peek() Returns the element at the front of the queue.
    boolean empty() Returns true if the queue is empty, false otherwise.

Notes:
    You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
    Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
*/

// Approach:
// 1) We use two vectors for the stacks. The first stack is the primary storage
// 2) For pushing, we simply push the given element into the first stack
// 3) For popping, we first pop all elements except the last into stack2, then we pop and store the remaining element
//    Then we pop all elements from stack2 back into stack1 and return the element we picked out
// 4) For peeking, we follow similar process to popping but instead of popping the last element we just get it
// 5) For empty, we just check if stack1 is empty

struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue{stack1: Vec::new(),stack2: Vec::new()}
    }
    
    fn push(&mut self, x: i32) {
        self.stack1.push(x)
    }
    
    fn pop(&mut self) -> i32 {
        while self.stack1.len() > 1 {
            self.stack2.push(self.stack1.pop().unwrap());
        }
        let x = self.stack1.pop().unwrap_or(-1);
        while self.stack2.len() > 0{
            self.stack1.push(self.stack2.pop().unwrap());
        }
        x
    }
    
    fn peek(&mut self) -> i32 {
        while self.stack1.len() > 1 {
            self.stack2.push(self.stack1.pop().unwrap());
        }
        let x = *self.stack1.get(self.stack1.len()-1).unwrap_or(&-1);
        while self.stack2.len() > 0{
            self.stack1.push(self.stack2.pop().unwrap());
        }
        x
    }
    
    fn empty(&self) -> bool {
        self.stack1.len() == 0
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
