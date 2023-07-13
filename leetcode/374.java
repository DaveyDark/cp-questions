/*
Question:
We are playing the Guess Game. The game is as follows:
I pick a number from 1 to n. You have to guess which number I picked.
Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
You call a pre-defined API int guess(int num), which returns three possible results:
    -1: Your guess is higher than the number I picked (i.e. num > pick).
    1: Your guess is lower than the number I picked (i.e. num < pick).
    0: your guess is equal to the number I picked (i.e. num == pick).
Return the number that I picked.
*/

// Approach:
// 1) Set the high and low limits to 0 and n, and init x to the mid point of them. Store result of api call in y
// 2) While the result is not 0, we keep updating x and y, and also updating the limits depending on the result
//    If the result is -1, our number(x) is higher, so we lower the high limit to x-1
//    If the result is 1, our number is lower, so we increase the lower limit to x+1
// 3) At the end we return the last value of x since at that value the api call returned 0

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * int guess(int num);
 */

public class Solution extends GuessGame {
    public int guessNumber(int n) {
        int low = 0, high = n;
        int x = low + (high-low)/2, y = guess(x);
        while(y != 0) {
            if(y == -1) {
                high = x-1;
            } else {
                low = x+1;
            }
            x = low + (high-low)/2;
            y = guess(x);
        }
        return x;
    }
}
