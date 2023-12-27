/*
Question:
Design an algorithm that collects daily price quotes for some stock and returns the span of that stock's price for the current day.

The span of the stock's price in one day is the maximum number of consecutive days
(starting from that day and going backward) for which the stock price was less than or equal to the price of that day.
    For example, if the prices of the stock in the last four days is [7,2,1,2] and the price of the stock today is 2,
    then the span of today is 4 because starting from today, the price of the stock was less than or equal 2 for 4 consecutive days.
    Also, if the prices of the stock in the last four days is [7,34,1,2] and the price of the stock today is 8,
    then the span of today is 3 because starting from today, the price of the stock was less than or equal 8 for 3 consecutive days.

Implement the StockSpanner class:
    StockSpanner() Initializes the object of the class.
    int next(int price) Returns the span of the stock's price given that today's price is price.
*/

// Approach:
// 1) Create a stack of tuples (price, count) and a counter in the struct.
// 2) In new(), initialize the stack and counter and return the struct.
// 3) in next(), while the stack is not empty and the top of the stack is less than the current
//    price, pop the stack. Then calculate the span by subtracting the count of the top of the
//    stack from the counter, or counter + 1 if the stack is empty.
//    Then push the current price and counter to the stack and increment the counter
//    Return the span.

struct StockSpanner {
    stocks: Vec<(i32, i32)>,
    cnt: i32,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner {
            stocks: vec![],
            cnt: 0,
        }
    }
    fn next(&mut self, price: i32) -> i32 {
        while let Some(top) = self.stocks.last() {
            if top.0 > price {
                break;
            }
            self.stocks.pop();
        }
        let span = self
            .stocks
            .last()
            .map_or(self.cnt + 1, |&(_, c)| self.cnt - c);
        self.stocks.push((price, self.cnt));
        self.cnt += 1;
        span
    }
}
