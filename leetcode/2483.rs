/*
Question:
You are given the customer visit log of a shop represented by a 0-indexed string customers consisting only of characters 'N' and 'Y':
    if the ith character is 'Y', it means that customers come at the ith hour
    whereas 'N' indicates that no customers come at the ith hour.

If the shop closes at the jth hour (0 <= j <= n), the penalty is calculated as follows:
    For every hour when the shop is open and no customers come, the penalty increases by 1.
    For every hour when the shop is closed and customers come, the penalty increases by 1.

Return the earliest hour at which the shop must be closed to incur a minimum penalty.
Note that if a shop closes at the jth hour, it means the shop is closed at the hour j.
*/

// Approach:
// 1) Make variables for prefix sum and init it to 0 and suffix sum and init it to teh count of
//    'Y's in the string
//    Also make variables to track the minimum sum and it's index, initialized to the sum at the
//    0th index i.e. suffix
// 2) Enumerate through the characters in the string
//    Match the current char and increase prefix if it is N or decrease suffix if it is Y
//    Then compare the current sum with the min sum and update min_sum and min_idx if needed
// 3) Return the min_idx at the end

impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut prefix = 0;
        let mut suffix = customers.chars().filter(|&x| x=='Y').count();
        let mut min_idx = 0;
        let mut min_sum = suffix;
        for (i,ch) in customers.chars().enumerate() {
            match ch {
                'N' => { prefix += 1 }
                'Y' => { suffix -= 1 }
                _ => {}
            };
            if min_sum > prefix+suffix {
                min_sum = prefix+suffix;
                min_idx = i as i32 + 1;
            }
        }
        min_idx
    }
}
