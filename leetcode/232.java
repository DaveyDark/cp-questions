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
// Use two stacks, s1 and s2
// push():
//   push to s1
// pop():
//   move all but last element from s1 to s2
//   pop andstore last element in ret
//   move back all elements from s2 to s1
//   return ret
// peek():
//   move all but last element from s1 to s2
//   store last element in ret
//   move back all elements from s2 to s1
//   return ret
// empty():
//  return s1.isEmpty()

class MyQueue {
  public Stack<Integer> s1, s2;

  public Sta
    public MyQueue() {
        this.s1 = new Stack<>();
        this.s2 = new Stack<>();
    }

  public void push(int x) {
    this.s1.push(x);
  }

  public int pop() {
    while (this.s1.size() > 1) {
      this.s2.push(this.s1.pop());
    }
    int ret = this.s1.pop();
    while (!this.s2.isEmpty()) {
      this.s1.push(this.s2.pop());
    }
    return ret;
  }

  public int peek() {
    while (this.s1.size() > 1) {
      this.s2.push(this.s1.pop());
    }
    int ret = this.s1.peek();
    while (!this.s2.isEmpty()) {
      this.s1.push(this.s2.pop());
    }
    return ret;
  }

  public boolean empty() {
    return this.s1.isEmpty();
  }
}
