/*
Question:
You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.
Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, 
return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.
*/

// Approach:
// 1) We go over the given array and check the ith element and it's adjacent elements
//    Unless i is the first element, we check if the left element is 0
//    and we check if i itself is 0
//    and unless i is the last element, we check if the right element is also 0
// 2) If all of these conditions are true then we plant a flower at i and decrease n
// 3) At then end if n <= 0 then all flowers were planted and we return true, other wise we return false

class Solution {
    public boolean canPlaceFlowers(int[] flowerbed, int n) {
        for(int i=0; i<flowerbed.length; i++) {
            if ((i==0 || flowerbed[i-1] == 0)
            && (flowerbed[i] == 0)
            && (i==flowerbed.length-1 || flowerbed[i+1] == 0)) {
                flowerbed[i] = 1;
                n--;
            }
        }
        return (n<=0);
    }
}
