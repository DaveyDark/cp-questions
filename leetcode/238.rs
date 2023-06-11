/*
Question:
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
You must write an algorithm that runs in O(n) time and without using the division operation.
*/

// Approach:
// 1) We first iterate over nums once to count the number of zeros and calculate product(of non-zero elements)
//    If the current number is 0 we increment zeros otherwise we multiply it to product
// 2) Then if there are 2 or more zeros un the array we return a 0 array of given length since all products will be 0
// 3) Then we go over nums and set nums[i]
//    ( We use in-place substitution to save memory)
//    If it's zero then we set it to the product
//    If it's non-zero and there is a zero in the array, we set it to 0
//    (because we know there is a max of 1 zero in the array, and if there is only 1 zerom
//    all elements except the 0 will have product_except_self as 0)
//    If it's non-zero and the array doesn't have a zero, we set it to product/nums[i]
// 4) We return the nums array back at the end

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut zeros = 0;
        let mut product = 1;
        let mut nums = nums;
        for num in nums.iter() {
            if *num == 0 { zeros+=1 }
            else {
                product *= *num;
            }
        }
        if zeros > 1 {return vec!(0; nums.len()) }
        for i in 0..nums.len() {
            nums[i] = 
                if nums[i] == 0 { product }
                else {
                    if zeros > 0 {0}
                    else {product/ nums[i]}
                }
            ;
        }
        nums
    }
}
