/*
Question:
Given an integer array arr and a filtering function fn, return a new array with a fewer or equal number of elements.
The returned array should only contain elements where fn(arr[i], i) evaluated to a truthy value.
Please solve it without the built-in Array.filter method.
*/

function filter(arr: number[], fn: (n: number, i: number) => any): number[] {
    let ans: number[] = [];
    for(let i=0; i<arr.length; i++){
        if(fn(arr[i],i))ans.push(arr[i]);
    }
    return ans;
};
