/*
Question:
Design a system that manages the reservation state of n seats that are numbered from 1 to n.

Implement the SeatManager class:
    SeatManager(int n) Initializes a SeatManager object that will manage n seats numbered from 1 to n. All seats are initially available.
    int reserve() Fetches the smallest-numbered unreserved seat, reserves it, and returns its number.
    void unreserve(int seatNumber) Unreserves the seat with the given seatNumber.
*/

// Approach
// SeatManager:
//  This struct will have 2 fields, a heap to store the unreserved seats which were previously
//  booked and an index i to store the current start index of unreserved unbooked seats
//  
//  new():
//      Creates and returns a new SeatManager object initialized with an empty heap and i as 0
//  reserve():
//      If we have any unreserved seats in the heap then we pop and return it
//      Otherwise we increment i and return it
//  unreserve():
//      We push the given seat into the heap

use std::collections::BinaryHeap;

struct SeatManager {
    heap: BinaryHeap<i32>,
    i: i32,
}

impl SeatManager {

    fn new(n: i32) -> Self {
        SeatManager {
            heap: BinaryHeap::new(),
            i: 0,
        }
    }
    
    fn reserve(&mut self) -> i32 {
        if self.heap.len() > 0 {
            return -self.heap.pop().unwrap();
        } else {
            self.i += 1;
            return self.i;
        }
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        self.heap.push(-seat_number);
    }
}
