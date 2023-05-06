/*
Question:
Given an integer n, return a counter function. This counter function initially returns n and then returns 1 more than the previous value every subsequent time it is called (n, n + 1, n + 2, etc).
*/

// Approach:
// We just return n++, which will return the original value but also increment the value after returning it

function createCounter(n: number): () => number {
    return function() {
        return n++;
    }
}
