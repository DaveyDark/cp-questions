/*
Question:
Given a function fn, return a new function that is identical to the original function except that it ensures fn is called at most once.
    The first time the returned function is called, it should return the same result as fn.
    Every subsequent time it is called, it should return undefined.
*/

function once<T extends (...args: any[]) => any>(fn: T): 
 ((...args: Parameters<T>) => ReturnType<T> | undefined) {
   let flag = false;
  return function (...args) {
      if(flag)return undefined;
      else {
        flag = true;
        return fn(...args);
      }
  };
}
