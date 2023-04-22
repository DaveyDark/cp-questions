/*
Question:
You are a product manager and currently leading a team to develop a new product. 
Unfortunately, the latest version of your product fails the quality check. 
Since each version is developed based on the previous version, all the versions after a bad version are also bad.
Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. 
You should minimize the number of calls to the API.
*/

// Approach:
// 1) We perform binary search in the range 1 to n
// 2) Each iteration we update left and right as needed while keeping track of the last bad version
// 3) At the end we return the last bad version

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = n;
        let mut last: i32 = 1;
        while left <= right {
            let mid = left + (right-left) /2;
            if self.isBadVersion(mid){
                right = mid-1;
                last = mid;
            } else {
                left = mid+1;
            }
        }
        last
    }
}
