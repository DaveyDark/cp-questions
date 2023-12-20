/*
Question:
You are given an integer array prices representing the prices of various chocolates in a store.
You are also given a single integer money, which represents your initial amount of money.

You must buy exactly two chocolates in such a way that you still have some non-negative leftover money.
You would like to minimize the sum of the prices of the two chocolates you buy.

Return the amount of money you will have leftover after buying the two chocolates.
If there is no way for you to buy two chocolates without ending up in debt, return money.
Note that the leftover must be non-negative.
*/

// Approach:
// 1) Make two variables to store the smallest two numbers.
// 2) Iterate over the array and update the variables accordingly.
// 3) Calculate the balance money.
// 4) If the balance money is greater than or equal to zero,
//    return the balance money, else return the money.

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut smallest1 = i32::MAX;
        let mut smallest2 = i32::MAX;

        for price in prices {
            if price < smallest2 {
                if price < smallest1 {
                    smallest2 = smallest1;
                    smallest1 = price;
                } else {
                    smallest2 = price;
                }
            }
        }

        let bal = money - (smallest1 + smallest2);

        if (bal >= 0) {
            bal
        } else {
            money
        }
    }
}
