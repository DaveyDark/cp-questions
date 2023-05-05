/*
Question:
Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
*/

// Approach:
// 1) We use a sliding window. start marks the start of the window and i marks the end
// 2) We go over the chars in the string and if the current substring length >= k then we decrease vowels and increase start to move the window to the right
// 3) If the current char is a vowel we increment vowels
// 4) If vowels in current window is greater than the max we set the max to this value
// 5) We return max at the end

function isVowel(s: string): boolean{
    if(s == 'a' || s == 'e' || s == 'i' || s == 'o' || s == 'u') {
        return true;
    }
    return false;
}

function maxVowels(s: string, k: number): number {
    let max: number = 0, vowels: number = 0,start: number = 0;
    for (let i = 0; i < s.length; i++) {
        if (i-start >= k) 
            if(isVowel(s[start++]))vowels--;
        if(isVowel(s[i])) vowels++;
        if(vowels > max)max = vowels;
    }
    return max;
};
