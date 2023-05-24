/*
Question:
Implement pow(x, n), which calculates x raised to the power n (i.e., xn).
*/

// Approach:
// If we sqauare root any even power, it is divided by two
// Using this principle, we check if the power is odd or even, and halve it if it is odd
// or just decrease it by 1 of it's odd to make it even
// 1) We make x mutable, and make a mutable, positive copy of n, and a ans variable to store ans
// 2) We loop until pow is not zero, and in each iteration we check if pow is even or odd
//    If it is odd we just multiply ans by x to increase power by one and decrement pow
//    If it is even we square x and divide power by 2
// 3) At the end we check if power is positive and return ans if it is or return 1/ans if it isn't

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut pow = n.abs() as u32;
        let mut ans = 1f64;
        while pow > 0 {
            if pow%2 == 1 {
                ans*=x;
                pow-=1;
            } else {
                x*=x;
                pow/=2;
            }
        }
        if(n>=0) {ans}
        else {1f64/ans}
    }
}
