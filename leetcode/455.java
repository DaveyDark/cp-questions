/*
Question:
Assume you are an awesome parent and want to give your children some cookies. 
But, you should give each child at most one cookie.

Each child i has a greed factor g[i], 
which is the minimum size of a cookie that the child will be content with; 
and each cookie j has a size s[j]. If s[j] >= g[i], 
we can assign the cookie j to the child i, and the child i will be content. 
Your goal is to maximize the number of your content children and output the maximum number.
*/

// Approach:
// 1) Sort both arrays
// 2) Make two pointers i and j for g and s respectively
// 3) Iterate over both arrays and increment i if g[i] <= s[j]
// 4) Return i

import java.util.*;

class Solution {
    public int findContentChildren(int[] g, int[] s) {
        Arrays.sort(g);
        Arrays.sort(s);
        int i = 0;
        for(int j = 0; i < g.length && j < s.length; j++) {
            if(g[i] <= s[j]) i++;
        }
        return i;
    }
}
