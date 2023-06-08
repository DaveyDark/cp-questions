/*
Question:
At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). 
Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. 
You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
Note that you do not have any change in hand at first.
Given an integer array bills where bills[i] is the bill the ith customer pays, 
return true if you can provide every customer with the correct change, or false otherwise.
*/

// Approach:
// 1) We make a vec to store the quantity of different bills we have
// 2) We go over the transactions one by one
//    If it's $5 we just increase bills[0]
//    If it's $10 then we increase bills[1] and decrease bills[0], or just return false if bills[0] is 0
//    If it's $20 then we return false if we dont have 3 $5 bills or 1 $10 and 1 $5 bill
//    If we do then we check and decrease the number of bills
// 3) If we reach outside the loop then we have completed all transactions successfully and so we return true

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut bal = vec!(0,0,0);
        for bill in bills.iter() {
            match bill {
                5 => {
                    bal[0] += 1;
                }
                10 => {
                    if bal[0] < 1 {return false;}
                    bal[0] -= 1;
                    bal[1] += 1;
                }
                20 => {
                    if !((bal[0] >= 3) || (bal[1] >= 1 && bal[0] >= 1)) {return false;}
                    if bal[1] >= 1 {
                        bal[1] -= 1;
                        bal[0] -= 1;
                    } else {
                        bal[0] -= 3;
                    }
                }
                _ => (),
            }
        }
        true
    }
}
