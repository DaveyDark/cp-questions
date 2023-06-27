/*
Question:
You have a RecentCounter class which counts the number of recent requests within a certain time frame.
Implement the RecentCounter class:
    RecentCounter() Initializes the counter with zero recent requests.
    
    int ping(int t) Adds a new request at time t, where t represents some time in milliseconds, 
    and returns the number of requests that has happened in the past 3000 milliseconds (including the new request). 
    Specifically, return the number of requests that have happened in the inclusive range [t - 3000, t].
It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.
*/

// Approach:
// We use a deque data structure to represent the requests log
//
// When a object is created, we just set reqs to a new Deque and return a new object
//
// When a ping is received, we push t to the back of the deque
// Then intil the first element in the deque is < t-3000, we keep popping an element from the front, this removes obselete pings from the deque to help reduce memory load
// Then we just return the length of reqs since it only contains recent pings

use std::collections::VecDeque;

struct RecentCounter {
    reqs: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            reqs: VecDeque::new(),
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.reqs.push_back(t);
        loop {
            if *self.reqs.front().unwrap() < (t-3000) {self.reqs.pop_front();}
            else {break}
        }
        self.reqs.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
