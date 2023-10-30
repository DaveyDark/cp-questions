/*
Question:
You are given an integer array arr. Sort the integers in the array in ascending order by the number of 1's in 
their binary representation and in case of two or more integers have the same 
number of 1's you have to sort them in ascending order.

Return the array after sorting it.
*/

// Approach:
// 1) Make a 2d vector to store the sorted by bits array where ith vec is the list of numbers with
//    i bits
// 2) Loop over all numbers in the array
// 3) For each number, calculate the number of bits and push it to the appropriate index in the 2d
//    vec and sort that row
// 4) Flatten the vec with concat() and return it

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut vec = vec!(vec!());
        for num in arr {
            let mut bits = 0;
            let mut n = num;
            while n > 0 {
                bits += n%2;
                n = n >> 1;
            }
            while bits as usize >= vec.len() { vec.push(vec!()) }
            vec[bits as usize].push(num);
            vec[bits as usize].sort_unstable();
        }
        vec.concat()
    }
}
