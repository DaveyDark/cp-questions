use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        self.q1.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.q1.len() == 0 {return -1;}
        while self.q1.len() > 1 {
            self.q2.push_back(self.q1.pop_front().unwrap());
        }
        let ret = self.q1.pop_front().unwrap();
        while self.q2.len() > 0 {
            self.q1.push_back(self.q2.pop_front().unwrap());
        }
        return ret;
    }
    
    fn top(&self) -> i32 {
        *self.q1.back().unwrap_or(&-1)
    }
    
    fn empty(&self) -> bool {
        self.q1.len() == 0
    }
}

