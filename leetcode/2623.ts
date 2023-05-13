/*
Question:
Given a function fn, return a memoized version of that function.
A memoized function is a function that will never be called twice with the same inputs. Instead it will return a cached value.
You can assume there are 3 possible input functions: sum, fib, and factorial.
    sum accepts two integers a and b and returns a + b.
    fib accepts a single integer n and returns 1 if n <= 1 or fib(n - 1) + fib(n - 2) otherwise.
    factorial accepts a single integer n and returns 1 if n <= 1 or factorial(n - 1) * n otherwise.
*/

// Approach:
// 1) We make a key for the cache by making a string out of the given args and then get the value at that key
// 2) If they value exists then we return it
// 3) Otherwise we compute and store the value in the cache, then return it

type Fn = (...params: any) => any

function memoize(fn: Fn): Fn {
    let cache: {[key: string]: any} = {};
    return function(...args) {
        let key = args.join("#").toString();
        let val = cache[key];
        if (val !== undefined) return val;
        const result = fn(...args);
        cache[key] = result;
        return result;
    }
}


/** 
 * let callCount = 0;
 * const memoizedFn = memoize(function (a, b) {
 *	 callCount += 1;
 *   return a + b;
 * })
 * memoizedFn(2, 3) // 5
 * memoizedFn(2, 3) // 5
 * console.log(callCount) // 1 
 */
