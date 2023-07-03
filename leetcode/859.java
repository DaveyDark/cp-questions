/*
Question:
Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.
Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].
    For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
*/

// Approach:
// This questions involves a lot of edge cases and most of the code is handling that
// 1) FIrst we return false if the lengths are unequal, no need to check anything else
// 2) Otherwise we make a hashset, a variable to track the swapping index,
//    and two booleans to indicated weather we have swapped and weather there is a duplicate char
// 3) Then we iterate over the strings. If the characters are unequal, we set swap index if it isn't set or we swap if we haven't swapped and it's possible
//    If we already swapped or can't swap this mismatch, we return false
//    Then we check if the current char is in the hashset, and mark duplicate as true if it is
//    Otherwise we add the char to the hashset
// 4) At the end, we return true if either a swap has been made or if no swap was made but the array has a duplicate

import java.util.*;

class Solution {
    public boolean buddyStrings(String s, String goal) {
        if(s.length() != goal.length()) return false;
        int swp = -1;
        Set<Character> set = new HashSet<Character>();
        boolean duplicate = false, swapped= false;
        for(int i=0; i<s.length(); i++) {
            if(s.charAt(i) != goal.charAt(i)){
                if(swp==-1) swp = i;
                else if(goal.charAt(swp)==s.charAt(i) && s.charAt(swp)==goal.charAt(i) && !swapped) swapped = true;
                else return false;
            }
            if(set.contains(s.charAt(i))) duplicate = true;
            else set.add(s.charAt(i));
        }

        return swapped || (duplicate && swp == -1);
    }
}
