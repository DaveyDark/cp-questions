/*
Question:
You have a set which contains all positive integers [1, 2, 3, 4, 5, ...].

Implement the SmallestInfiniteSet class:
    SmallestInfiniteSet() Initializes the SmallestInfiniteSet object to contain all positive integers.
    int popSmallest() Removes and returns the smallest integer contained in the infinite set.
    void addBack(int num) Adds a positive integer num back into the infinite set, if it is not already in the infinite set.
*/

// Approach:
// SmallestInfiniteSet:
//  This struct contains a BinaryHeap to store the re-added numbers and a i32 start to store the
//  start of the infinite set
//  new():
//      Returns a SmallestInfiniteSet object with an empty heap and start set to 1
//  pop_smallest():
//      If the heap is filled then we pop an element and return it
//      Otherwise we increment the start and return it's previous value
//  add_back():
//      If the element is outside the range of the infinite numbers and the heap doesn't already
//      contain it, we push the number into the heap. We apply a negative sign to use BinaryHeap as
//      a MinHeap since it's a max heap by default

use std::collections::BinaryHeap;

struct SmallestInfiniteSet {
    heap: BinaryHeap<i32>,
    start: i32,
}

impl SmallestInfiniteSet {

    fn new() -> Self {
        SmallestInfiniteSet {
            heap: BinaryHeap::new(),
            start: 1,
        }
    }
    
    fn pop_smallest(&mut self) -> i32 {
        if self.heap.len() > 0 {
            -self.heap.pop().unwrap()
        } else {
            self.start += 1;
            self.start - 1
        }
    }
    
    fn add_back(&mut self, num: i32) {
        if num < self.start && !self.heap.iter().any(|&x| x == -num) {
            self.heap.push(-num);
        }
    }
}
