/*
Question:
You are given an array of unique integers salary where salary[i] is the salary of the ith employee.
Return the average salary of employees excluding the minimum and maximum salary. Answers within 10-5 of the actual answer will be accepted.
*/

// Approach:
// 1) We make variables to track the max, min and sum of the array
// 2) We go through the array, updating max to store the maximum value,
//    min to store the minimum value and sum to store the sum
//    Since int can only store upto 2^31 and the salaries are in magnitudes of 1000,
//    We divide salary by 1000 before adding to sum for optimization
// 3) Then we subtract the minimum and maximum salaries, divided by 1000, from sum
// 4) Then we return the average of remaining salaries as a double,
//    which is the sum divided by the length of salaries -2

class Solution {
    public double average(int[] salary) {
        int max = salary[0],min = salary[0];
        int sum = 0;
        for(int sal: salary) {
            if(sal < min) min = sal;
            if(sal > max) max = sal;
            sum += sal/1000;
        }
        sum -= (min/1000) + (max/1000);
        return ((double)sum/(salary.length - 2))*1000;
    }
}
